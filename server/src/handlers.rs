
use axum::{
    extract::State,
    http::StatusCode,
    Json
};


use sqlx::postgres::PgPool;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct NewProduct {
    name: String,
    price: i32
}


#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Product {
    id: i32,
    name: String,
    price: i32
}

pub async fn create_product(State(pool): State<PgPool>, Json(product): Json<NewProduct>) -> Result<Json<Value>, StatusCode> {
    let _result = sqlx::query("INSERT INTO products (name, price) values ($1, $2)")
    .bind(&product.name)
    .bind(&product.price)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(json!(product)))
}


pub async fn get_products (State(pool): State<PgPool>) -> Result<Json<Vec<Product>>,(StatusCode, String)> {
let result = sqlx::query_as("SELECT * FROM products")
            .fetch_all(&pool)
            .await
            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Somethin is wrong {}", err)))?;
            Ok(Json(result))
}


pub async fn get_one_product(State(pool): State<PgPool>,
                            axum::extract::Path(id): axum::extract::Path<i32> )
                            -> Result<Json<Product>,(StatusCode, String)> {
let result = sqlx::query_as(
        "SELECT id, name, price FROM products WHERE id = $1")  
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
                format!("Error is {}", err)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something is wrong {}", err))
        })?;
        Ok(Json(result))


}

pub async fn delete_product(State(pool): State<PgPool>,
                                axum::extract::Path(id): axum::extract::Path<i32> )
                                -> Result<Json<Value>,(StatusCode, String)> {
    let result = sqlx::query(
            "DELETE FROM products WHERE id = $1")  
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND,
                    format!("Error is {}", err)),
                _ => (StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Something is wrong {}", err))
            })?;
            Ok(Json(json!({"msg": "product deleted successfully"})))


}



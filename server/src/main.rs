use axum::
{routing::{get,post, get_service},

Router};
use http::{ Method, header};

use tower_http::{
        cors::{Any, CorsLayer},
        services::{ServeDir, ServeFile}
};

mod handlers;

#[tokio::main]    
async fn main() {


//tracing
tracing_subscriber::fmt::init();


//add database
dotenv::dotenv().ok();
let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
let pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Error with pool connection");


//add table if it does not exist
sqlx::query(
    r#"
        CREATE TABLE IF NOT EXISTS products (
            id serial,
            name text,
            price integer
        );"#,
)
.execute(&pool)
.await;

// add cors
let cors = CorsLayer::new()
                        .allow_origin(Any);


let app = Router::new()
        .route("/", get(home))
        .layer(cors)
        .route("/api/products", get(handlers::get_products).post(handlers::create_product))
        .route("/api/products/:id", get(handlers::get_one_product).delete(handlers::delete_product))
        .with_state(pool);


tracing::debug!("debut printing: listening on port {}", "0.0.0.0:3000");
println!(" Listening on port {}" , "0.0.0.0:3000" );
axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn home() -> &'static str {
"Home Page here !"
}



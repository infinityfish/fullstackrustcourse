===================================
#### Connecting frontend and server ####
create and add to Cargo.toml
[workspace]

members = [
"server", "frontend"
]

#create and add to .gitignore
target/
dist/
.env

create frontend/Trunk.toml and add contents
[build]
target = "index.html"
dist = "../dist"

now you can run: trunk serve

# now go to server/main.rs
axum::response::IntoResponse
axum::http::StatusCode,
use std::io;

//serving frontend static files
let serve_dir = ServeDir::new("../frontend/dist").not_found_service(ServeFile::new("../dist/frontend/index.html"));
let serve_dir = get_service(serve_dir).handle_error(handle_error);

# add to router

        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone());

# add handle error fn    
async fn handle_error(_err: io::Error) -> impl IntoResponse {
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
    }

  # change current / to /home
cd into backend: cargo watch -x run
=========================================
use std::env;
use axum::{ http::StatusCode, response::IntoResponse, routing::{ get, post }, Json, Router };
use serde::{ Deserialize, Serialize };
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    /* Logger */
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let addr = SocketAddr::from(([127,0,0,1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::debug!("listening on {}", addr);
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{ header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, world!");
    }
}

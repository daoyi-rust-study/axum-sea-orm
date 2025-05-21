use axum::{Router, routing::get};

pub mod posts;

pub fn routes() -> Router {
    Router::new().nest("/posts", posts_api())
}

fn posts_api() -> Router {
    Router::new().route("/", get(posts::index_handler))
}

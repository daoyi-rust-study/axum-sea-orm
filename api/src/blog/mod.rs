use crate::StateRouter;
use axum::{
    Router,
    routing::{get, post},
};

pub mod posts;

pub fn routes() -> StateRouter {
    Router::new().nest("/posts", posts_api())
}

fn posts_api() -> StateRouter {
    Router::new()
        .route("/", get(posts::index_handler))
        .route("/query", get(posts::from_query))
        .route("/body", post(posts::from_body))
}

use crate::StateRouter;
use axum::{routing::get, Router};

pub mod posts;

pub fn routes() -> StateRouter {
    Router::new().nest("/posts", posts_api())
}

fn posts_api() -> StateRouter {
    Router::new().route("/", get(posts::index_handler))
}

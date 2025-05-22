use crate::StateRouter;
use axum::{
    Router,
    routing::{delete, get, post, put},
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
        .route("/create_test", post(posts::create_first_posts))
        .route("/list", get(posts::query_list))
        .route("/create", post(posts::create))
        .route("/update", put(posts::update))
        .route("/findById", get(posts::query_by_id))
        .route("/deleteById", delete(posts::delete_by_id))
}

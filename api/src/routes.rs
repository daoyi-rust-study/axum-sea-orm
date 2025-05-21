use axum::Router;
use crate::{blog, StateRouter};

pub fn compose() -> StateRouter {
    Router::new().nest("/blog", blog::routes())
}
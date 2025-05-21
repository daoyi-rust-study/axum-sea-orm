use axum::Router;
use crate::blog;

pub fn compose() -> Router {
    Router::new().nest("/blog", blog::routes())
}
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct Res<T> {
    code: i32,
    data: Option<T>,
    message: String,
}

impl<T> Res<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }
    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            data: None,
            message,
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> Response {
        let val = json!(self);
        Json(val).into_response()
    }
}

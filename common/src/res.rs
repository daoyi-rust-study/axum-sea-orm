use crate::error::CusErr;
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
    pub fn error(e: anyhow::Error) -> Self {
        let code = if e.downcast_ref::<CusErr>().is_some() {
            match e.downcast_ref::<CusErr>() {
                Some(CusErr::AppRuleError(_)) => 400,
                _ => 404,
            }
        } else {
            500
        };
        Self {
            code,
            data: None,
            message: e.to_string(),
        }
    }
}

impl<T: Serialize> IntoResponse for Res<T> {
    fn into_response(self) -> Response {
        let val = json!(self);
        Json(val).into_response()
    }
}

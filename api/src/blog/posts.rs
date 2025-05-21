use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use common::res::Res;

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    println!("{state:?}");

    #[derive(serde::Serialize)]
    struct Data {
        name: String,
        age: i32,
    }

    Res::success(Data {
        name: "blog".to_string(),
        age: 30,
    })

    // let res_json = ResJson {
    //     code: 200,
    //     data: "okk".to_string(),
    //     message: "ok".to_string(),
    // };
    //
    // res_json

    // let json_string = json!(res_json).to_string();
    // let mut res = json_string.into_response();
    // res.headers_mut().insert(
    //     header::CONTENT_TYPE,
    //     HeaderValue::from_str("application/json").unwrap(),
    // );
    // res

    // let header_part = AppendHeaders([(
    //     header::CONTENT_TYPE,
    //     HeaderValue::from_str("application/json").unwrap(),
    // )]);
    //
    // (header_part, json_string)
}

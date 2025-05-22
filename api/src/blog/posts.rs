use crate::AppState;
use anyhow::Error;
use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use common::{error::CusErr, res::Res};
use core::posts;
use serde::{Deserialize, Serialize};

pub async fn index_handler(state: State<AppState>) -> impl IntoResponse {
    println!("{state:?}");

    let res = test_service().await;

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }

    // #[derive(serde::Serialize)]
    // struct Data {
    //     name: String,
    //     age: i32,
    // }
    //
    // Res::success(Data {
    //     name: "blog".to_string(),
    //     age: 30,
    // })

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

/**
 * @description: 创建第一篇文章
 * @param {State<AppState>} state
 * @param {WithRejection<Json<CreatePostsParam>, Res<()>>} param
 * @return {*}
 */
pub async fn create_first_posts(
    state: State<AppState>,
    WithRejection(Json(param), _): WithRejection<Json<CreatePostsParam>, Res<()>>,
) -> impl IntoResponse {
    let res = posts::create_first_posts(&state.db, param.title).await;
    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}

/**
 * @description: 查询列表
 * @param {State<AppState>} state
 * @return {*}
 */
pub async fn query_list(
    state: State<AppState>,
    WithRejection(Query(param), _): WithRejection<Query<QueryListParams>, Res<()>>,
) -> impl IntoResponse {
    println!("params: {:?}", param);
    Res::<String>::empty_success()
}
/**
 * @description: 查询单个文章
 * @param {State<AppState>} state
 * @return {*}
 */
pub async fn query_by_id(state: State<AppState>) -> impl IntoResponse {
    Res::<String>::empty_success()
}
/**
 * @description: 创建文章
 * @param {State<AppState>} state
 * @return {*}
 */
pub async fn create(state: State<AppState>) -> impl IntoResponse {
    Res::<String>::empty_success()
}
/**
 * @description: 更新文章
 * @param {State<AppState>} state
 * @return {*}
 */
pub async fn update(state: State<AppState>) -> impl IntoResponse {
    Res::<String>::empty_success()
}
/**
 * @description: 删除文章
 * @param {State<AppState>} state
 * @return {*}
 */
pub async fn delete_by_id(state: State<AppState>) -> impl IntoResponse {
    Res::<String>::empty_success()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ByIdParams {
    /// ID 编号
    id: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    id: i32,
    text: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParams {
    text: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryListParams {
    page_number: Option<i32>,
    page_size: Option<i32>,
    /// 标题
    title: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePostsParam {
    title: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct A {
    a: String,
}

// from query
pub async fn from_query(
    WithRejection(Query(param), _): WithRejection<Query<A>, Res<()>>,
) -> impl IntoResponse {
    let data = format!("param from query a = {}", param.a);
    Res::success(data)
}

// from body
pub async fn from_body(
    WithRejection(Json(param), _): WithRejection<Json<A>, Res<()>>,
) -> impl IntoResponse {
    let data = format!("param from body a = {}", param.a);
    Res::success(data)
}

pub async fn test_service() -> anyhow::Result<String> {
    Err(Error::from(CusErr::AppRuleError(
        "对不起，触犯天条了！".into(),
    )))
}

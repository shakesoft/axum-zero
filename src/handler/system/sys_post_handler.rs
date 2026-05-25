use crate::service::system::sys_post_service::SysPostService;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加岗位信息表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn add_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<PostReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::add_sys_post(rb, item).await
}

/*
 *删除岗位信息表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<DeletePostReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::delete_sys_post(rb, item).await
}

/*
 *更新岗位信息表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_post(State(state): State<Arc<AppState>>, Json(item): Json<PostReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::update_sys_post(rb, item).await
}

/*
 *更新岗位信息表状态
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_post_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdatePostStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::update_sys_post_status(rb, item).await
}

/*
 *查询岗位信息表详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_post_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::query_sys_post_detail(rb, item).await
}

/*
 *查询岗位信息表列表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_post_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryPostListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysPostService::query_sys_post_list(rb, item).await
}

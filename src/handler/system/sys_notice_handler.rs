use crate::service::system::sys_notice_service::SysNoticeService;
use crate::vo::system::sys_notice_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加通知公告表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn add_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<NoticeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::add_sys_notice(rb, item).await
}

/*
 *删除通知公告表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<DeleteNoticeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::delete_sys_notice(rb, item).await
}

/*
 *更新通知公告表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_notice(State(state): State<Arc<AppState>>, Json(item): Json<NoticeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::update_sys_notice(rb, item).await
}

/*
 *更新通知公告表状态
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_notice_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateNoticeStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::update_sys_notice_status(rb, item).await
}

/*
 *查询通知公告表详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_notice_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::query_sys_notice_detail(rb, item).await
}

/*
 *查询通知公告表列表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_notice_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::query_sys_notice_list(rb, item).await
}

#[function_name::named]
pub async fn query_sys_notice_request(State(state): State<Arc<AppState>>, Json(item): Json<QueryNoticeListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysNoticeService::query_sys_notice_request(rb, item).await
}

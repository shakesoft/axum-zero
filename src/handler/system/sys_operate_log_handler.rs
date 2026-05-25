use crate::service::system::sys_operate_log_service::SysOperateLogService;
use crate::vo::system::sys_operate_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *删除操作日志记录
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_operate_log(State(state): State<Arc<AppState>>, Json(item): Json<DeleteOperateLogReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysOperateLogService::delete_sys_operate_log(rb, item).await
}

/*
 *清空操作日志记录
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn clean_sys_operate_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("{function_name}", function_name = function_name!());
    let rb = &state.batis;

    SysOperateLogService::clean_sys_operate_log(rb).await
}

/*
 *查询操作日志记录详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_operate_log_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryOperateLogDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysOperateLogService::query_sys_operate_log_detail(rb, item).await
}
/*
 *查询操作日志记录列表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_operate_log_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryOperateLogListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysOperateLogService::query_sys_operate_log_list(rb, item).await
}

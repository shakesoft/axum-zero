use crate::service::system::sys_login_log_service::SysLoginLogService;
use crate::vo::system::sys_login_log_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *删除系统访问记录
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_login_log(State(state): State<Arc<AppState>>, Json(item): Json<DeleteLoginLogReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysLoginLogService::delete_sys_login_log(rb, item).await
}

/*
 *清空系统登录日志
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn clean_sys_login_log(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("{function_name}", function_name = function_name!());
    let rb = &state.batis;

    SysLoginLogService::clean_sys_login_log(rb).await
}

/*
 *查询系统访问记录详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_login_log_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysLoginLogService::query_sys_login_log_detail(rb, item).await
}

/*
 *查询系统访问记录列表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_login_log_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryLoginLogListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysLoginLogService::query_sys_login_log_list(rb, item).await
}

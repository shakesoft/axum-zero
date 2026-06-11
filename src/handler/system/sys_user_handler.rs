use crate::aop::aspects::timer::Timer;
use crate::service::system::sys_user_service::SysUserService;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use aspect_macros::aspect;
use aspect_std::LoggingAspect;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use log::info;
use std::any::Any;
use std::net::SocketAddr;
use std::sync::Arc;

/*
 *添加用户信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn add_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::add_sys_user(rb, item).await
}

/*
 *删除用户信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn delete_sys_user(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<DeleteUserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::delete_sys_user(rb, &state.redis, headers, item).await
}

/*
 *更新用户信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::update_sys_user(rb, &state.redis, item).await
}

/*
 *更新用户信息状态
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::update_sys_user_status(rb, &state.redis, item).await
}

/*
 *重置用户密码
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn reset_sys_user_password(State(state): State<Arc<AppState>>, Json(item): Json<ResetUserPwdReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::reset_sys_user_password(rb, &state.redis, item).await
}

/*
 *用户修改自己的密码
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user_password(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::update_sys_user_password(rb, headers, item).await
}

/*
 *查询用户信息详情
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_user_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::query_sys_user_detail(rb, item).await
}

/*
 *查询用户信息列表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
// #[aspect(Timer)]
// #[aspect(Logger)]
#[aspect(LoggingAspect::new())]
pub async fn query_sys_user_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::query_sys_user_list(rb, item).await
}

/*
 *用户登录
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
// #[aspect(Timer)]
pub async fn login(headers: HeaderMap, ConnectInfo(remote_addr): ConnectInfo<SocketAddr>, State(state): State<Arc<AppState>>, Valid(Json(item)): Valid<Json<UserLoginReq>>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::login(rb, &state.redis, headers, remote_addr, item).await
}

/*
 *查询用户角色
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::query_user_role(rb, item).await
}

// 更新用户角色
#[function_name::named]
pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;
    SysUserService::update_user_role(rb, &state.redis, item).await
}

// 查询用户菜单
#[function_name::named]
pub async fn query_user_menu(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();
    info!("{function_name}:{user_id:?}", function_name = function_name!());

    let rb = &state.batis;
    SysUserService::query_user_menu(rb, &state.redis, headers).await
}

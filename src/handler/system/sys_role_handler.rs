use crate::service::system::sys_role_service::SysRoleService;
use crate::vo::system::sys_role_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
use aspect_macros::aspect;
use crate::aop::aspects::logger::Logger;

// use std::time::Duration;
// use tokio::time::sleep;
/*
 *添加角色信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn add_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<RoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::add_sys_role(rb, item).await
}

/*
 *删除角色信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn delete_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<DeleteRoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::delete_sys_role(rb, item).await
}

/*
 *更新角色信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_role(State(state): State<Arc<AppState>>, Json(item): Json<RoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::update_sys_role(rb, item).await
}

/*
 *更新角色信息状态
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_role_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::update_sys_role_status(rb, item).await
}

/*
 *查询角色信息详情
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_role_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::query_sys_role_detail(rb, item).await
}

/*
 *查询角色信息列表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
#[aspect(Logger)]
pub async fn query_sys_role_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::query_sys_role_list(rb, item).await
}

/*
 *查询角色关联的菜单
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
#[aspect(Logger)]
pub async fn query_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::query_role_menu(rb, item).await
}

/*
 *更新角色关联的菜单
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_role_menu(State(state): State<Arc<AppState>>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysRoleService::update_role_menu(rb, item).await
}

/*
 *查询已分配用户角色列表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_allocated_list(State(state): State<Arc<AppState>>, Json(item): Json<AllocatedListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());

    let rb = &state.batis;

    SysRoleService::query_allocated_list(rb, item).await
}

/*
 *查询未分配用户角色列表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_unallocated_list(State(state): State<Arc<AppState>>, Json(item): Json<UnallocatedListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());

    let rb = &state.batis;

    SysRoleService::query_unallocated_list(rb, item).await
}

/*
 *取消授权用户
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());

    let rb = &state.batis;

    SysRoleService::cancel_auth_user(rb, item).await
}

/*
 *批量取消授权用户
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn batch_cancel_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<CancelAuthUserAllReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());

    let rb = &state.batis;

    SysRoleService::batch_cancel_auth_user(rb, item).await
}

/*
 *批量选择用户授权
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn batch_auth_user(State(state): State<Arc<AppState>>, Json(item): Json<SelectAuthUserAllReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());

    let rb = &state.batis;

    SysRoleService::batch_auth_user(rb, item).await
}

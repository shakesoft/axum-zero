use crate::service::system::sys_menu_service::SysMenuService;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加菜单信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn add_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::add_sys_menu(rb, item).await
}

/*
 *删除菜单信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn delete_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<DeleteMenuReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::delete_sys_menu(rb, item).await
}

/*
 *更新菜单信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_menu(State(state): State<Arc<AppState>>, Json(item): Json<MenuReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::update_sys_menu(rb, item).await
}

/*
 *更新菜单信息状态
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_menu_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateMenuStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::update_sys_menu_status(rb, item).await
}

/*
 *查询菜单信息详情
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_menu_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::query_sys_menu_detail(rb, item).await
}

/*
 *查询菜单信息列表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_menu_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryMenuListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::query_sys_menu_list(rb, item).await
}

/*
 *查询菜单信息(排除按钮)
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_menu_list_simple(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("{function_name}", function_name = function_name!());
    let rb = &state.batis;

    SysMenuService::query_sys_menu_list_simple(rb).await
}

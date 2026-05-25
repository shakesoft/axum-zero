use crate::service::system::sys_dict_type_service::SysDictTypeService;
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加字典类型
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn add_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::add_sys_dict_type(rb, item).await
}

/*
 *删除字典类型
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::delete_sys_dict_type(rb, item).await
}

/*
 *更新字典类型
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::update_sys_dict_type(rb, item).await
}

/*
 *更新字典类型状态
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_type_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictTypeStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::update_sys_dict_type_status(rb, item).await
}

/*
 *查询字典类型详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_type_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::query_sys_dict_type_detail(rb, item).await
}

/*
 *查询字典类型列
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_type_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeService::query_sys_dict_type_list(rb, item).await
}

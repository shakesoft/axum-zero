use crate::service::system::sys_dict_data_service::SysDictDataService;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use std::sync::Arc;
/*
 *添加字典数据
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn add_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DictDataReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::add_sys_dict_data(rb, item).await
}

/*
 *删除字典数据
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictDataReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::delete_sys_dict_data(rb, item).await
}

/*
 *更新字典数据
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_data(State(state): State<Arc<AppState>>, Json(item): Json<DictDataReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::update_sys_dict_data(rb, item).await
}

/*
 *更新字典数据状态
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_data_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictDataStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::update_sys_dict_data_status(rb, item).await
}

/*
 *查询字典数据详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_data_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::query_sys_dict_data_detail(rb, item).await
}

/*
 *查询字典数据列
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_data_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictDataListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDictDataService::query_sys_dict_data_list(rb, item).await
}

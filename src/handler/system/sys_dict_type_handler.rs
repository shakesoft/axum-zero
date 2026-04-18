use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::{count_dict_data_by_type, update_dict_data_type};
use crate::model::system::sys_dict_type_model::DictType;
use crate::dao::system::sys_dict_type_dao::{SysDictTypeDao};
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbs::value;
use std::sync::Arc;
/*
 *添加字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn add_sys_dict_type(State(state): State<Arc<AppState>>, Json(mut item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    if DictType::select_by_dict_type(rb, &item.dict_type).await?.is_some() {
        return Err(AppError::BusinessError("字典类型已存在"));
    }

    item.id = None;
    DictType::insert(rb, &DictType::from(item)).await.map(|_| ok_result())?
}

/*
 *删除字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn delete_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DeleteDictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let ids = item.ids.clone();
    for id in ids {
        let p = match DictType::select_by_id(rb, &id).await? {
            None => return Err(AppError::BusinessError("字典类型不存在,不能删除")),
            Some(p) => p,
        };

        if count_dict_data_by_type(rb, &p.dict_type).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    DictType::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新字典类型
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_type(State(state): State<Arc<AppState>>, Json(item): Json<DictTypeReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let id = item.id;

    if item.id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    if DictType::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("字典类型不存在"));
    }

    if let Some(x) = DictType::select_by_dict_type(rb, &item.dict_type).await? {
        if x.id != id {
            return Err(AppError::BusinessError("字典类型已存在"));
        }

        let dict_type = x.dict_type;
        update_dict_data_type(rb, &*item.dict_type, &dict_type).await?;
    }

    DictType::update_by_map(rb, &DictType::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新字典类型状态
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn update_sys_dict_type_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDictTypeStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    SysDictTypeDao::update_dict_type_status(rb, item.status, &item.ids).await.map(|_| ok_result())?
}

/*
 *查询字典类型详情
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_type_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    DictType::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("字典类型不存在")),
        |x| {
            let data: DictTypeResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询字典类型列
 *author：刘飞华
 *date：2024/12/25 11:36:48
 */
#[function_name::named]
pub async fn query_sys_dict_type_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDictTypeListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let page = &PageRequest::new(item.page_no, item.page_size);

    DictType::select_dict_type_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictTypeResp>>(), x.total))?
}

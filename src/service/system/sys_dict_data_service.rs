use crate::common::error::{AppError,ServiceResultPage, ServiceResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::dao::system::sys_dict_data_dao::SysDictDataDao;
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::{DeleteDictDataReq, DictDataReq, DictDataResp, QueryDictDataDetailReq, QueryDictDataListReq, UpdateDictDataStatusReq};
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysDictDataService;

impl SysDictDataService {
    pub async fn add_sys_dict_data(rb: &RBatis, mut item: DictDataReq) -> ServiceResult<String> {
        if DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await?.is_some() {
            return Err(AppError::BusinessError("字典标签已存在"));
        }

        if DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await?.is_some() {
            return Err(AppError::BusinessError("字典键值已存在"));
        }

        item.id = None;
        DictData::insert(rb, &DictData::from(item)).await.map(|_| ok_result())?
    }

    pub async fn delete_sys_dict_data(rb: &RBatis, item: DeleteDictDataReq) -> ServiceResult<String> {
        DictData::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
    }

    pub async fn update_sys_dict_data(rb: &RBatis, item: DictDataReq) -> ServiceResult<String> {
        let id = item.id;

        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if DictData::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("字典数据不存在"));
        }

        if let Some(x) = DictData::select_by_dict_label(rb, &item.dict_type, &item.dict_label).await? {
            if x.id != id {
                return Err(AppError::BusinessError("字典标签已存在"));
            }
        }

        if let Some(x) = DictData::select_by_dict_value(rb, &item.dict_type, &item.dict_value).await? {
            if x.id != id {
                return Err(AppError::BusinessError("字典键值已存在"));
            }
        }

        DictData::update_by_map(rb, &DictData::from(item), value! {"id": &id}).await.map(|_| ok_result())?
    }

    pub async fn update_sys_dict_data_status(rb: &RBatis, item: UpdateDictDataStatusReq) -> ServiceResult<String> {
        SysDictDataDao::update_dict_data_status(rb, item.status, &item.ids).await.map(|_| ok_result())?
    }

    pub async fn query_sys_dict_data_detail(rb: &RBatis, item: QueryDictDataDetailReq) -> ServiceResult<DictDataResp> {
        DictData::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("字典数据不存在")),
            |x| {
                let data: DictDataResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_dict_data_list(rb: &RBatis, item: QueryDictDataListReq) -> ServiceResultPage<DictDataResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        DictData::select_dict_data_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictDataResp>>(), x.total))?
    }
}

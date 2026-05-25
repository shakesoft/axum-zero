use crate::common::error::{AppError, ServiceResult, ServiceResultPage};
use crate::common::result::{ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_dict_data_dao;
use crate::dao::system::sys_dict_type_dao::SysDictTypeDao;
use crate::model::system::sys_dict_type_model::DictType;
use crate::vo::system::sys_dict_type_vo::{DeleteDictTypeReq, DictTypeReq, DictTypeResp, QueryDictTypeDetailReq, QueryDictTypeListReq, UpdateDictTypeStatusReq};
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysDictTypeService;

impl SysDictTypeService {
    pub async fn add_sys_dict_type(rb: &RBatis, mut item: DictTypeReq) -> ServiceResult<String> {
        if DictType::select_by_dict_type(rb, &item.dict_type).await?.is_some() {
            return Err(AppError::BusinessError("字典类型已存在"));
        }

        item.id = None;
        DictType::insert(rb, &DictType::from(item)).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_dict_type(rb: &RBatis, item: DeleteDictTypeReq) -> ServiceResult<String> {
        let ids = item.ids.clone();
        for id in ids {
            let p = match DictType::select_by_id(rb, &id).await? {
                None => return Err(AppError::BusinessError("字典类型不存在,不能删除")),
                Some(p) => p,
            };

            if sys_dict_data_dao::count_dict_data_by_type(rb, &p.dict_type).await? > 0 {
                return Err(AppError::BusinessError("已分配,不能删除"));
            }
        }

        DictType::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_dict_type(rb: &RBatis, item: DictTypeReq) -> ServiceResult<String> {
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
            sys_dict_data_dao::update_dict_data_type(rb, &*item.dict_type, &dict_type).await?;
        }

        DictType::update_by_map(rb, &DictType::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_dict_type_status(rb: &RBatis, item: UpdateDictTypeStatusReq) -> ServiceResult<String> {
        SysDictTypeDao::update_dict_type_status(rb, item.status, &item.ids).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_dict_type_detail(rb: &RBatis, item: QueryDictTypeDetailReq) -> ServiceResult<DictTypeResp> {
        DictType::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("字典类型不存在")),
            |x| {
                let data: DictTypeResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_dict_type_list(rb: &RBatis, item: QueryDictTypeListReq) -> ServiceResultPage<DictTypeResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        DictType::select_dict_type_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictTypeResp>>(), x.total))?
    }
}

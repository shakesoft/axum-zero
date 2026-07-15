use crate::common::error::{AppError, ServiceResult};
use crate::common::result::{ok_result_empty, ok_result_data, Paged};
use crate::dao::system::sys_operate_log_dao;
use crate::model::system::sys_operate_log_model::OperateLog;
use crate::vo::system::sys_operate_log_vo::{DeleteOperateLogReq, OperateLogResp, QueryOperateLogDetailReq, QueryOperateLogListReq};
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysOperateLogService;

impl SysOperateLogService {
    pub async fn delete_sys_operate_log(rb: &RBatis, item: DeleteOperateLogReq) -> ServiceResult<String> {
        OperateLog::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn clean_sys_operate_log(rb: &RBatis) -> ServiceResult<String> {
        sys_operate_log_dao::clean_operate_log(rb).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_operate_log_detail(rb: &RBatis, item: QueryOperateLogDetailReq) -> ServiceResult<OperateLogResp> {
        OperateLog::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("操作日志不存在")),
            |x| {
                let data: OperateLogResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_operate_log_list(rb: &RBatis, item: QueryOperateLogListReq) -> ServiceResult<Paged<OperateLogResp>> {
        let page = &PageRequest::new(item.page_no, item.page_size);
        OperateLog::select_page_by_name(rb, page, &item)
            .await
            .map(|x| ok_result_data(Paged { total: x.total, items: x.records.into_iter().map(|x| x.into()).collect() }))?
    }
}

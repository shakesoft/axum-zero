use crate::common::error::{AppError, AppResult, ServiceResult, ServiceResultPage};
use crate::common::result::{ok, ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_notice_dao::SysNoticeDao;
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::{DeleteNoticeReq, NoticeReq, NoticeResp, QueryNoticeDetailReq, QueryNoticeListReq, UpdateNoticeStatusReq};
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysNoticeService;

impl SysNoticeService {
    pub async fn add_sys_notice(rb: &RBatis, mut item: NoticeReq) -> ServiceResult<String> {
        if Notice::select_by_title(rb, &item.notice_title).await?.is_some() {
            return Err(AppError::BusinessError("公告标题已存在"));
        };

        item.id = None;
        Notice::insert(rb, &Notice::from(item)).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_notice(rb: &RBatis, item: DeleteNoticeReq) -> ServiceResult<String> {
        Notice::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_notice(rb: &RBatis, item: NoticeReq) -> ServiceResult<String> {
        let id = item.id;
        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Notice::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("通知公告表不存在"));
        }

        if let Some(x) = Notice::select_by_title(rb, &item.notice_title).await? {
            if x.id != id {
                return Err(AppError::BusinessError("公告标题已存在"));
            }
        }

        Notice::update_by_map(rb, &Notice::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_notice_status(rb: &RBatis, item: UpdateNoticeStatusReq) -> ServiceResult<String> {
        SysNoticeDao::update_status(rb, &item.ids, item.status).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_notice_detail(rb: &RBatis, item: QueryNoticeDetailReq) -> ServiceResult<NoticeResp> {
        Notice::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("通知公告表不存在")),
            |x| {
                let notice: NoticeResp = x.into();
                ok_result_data(notice)
            },
        )
    }

    pub async fn query_sys_notice_list(rb: &RBatis, item: QueryNoticeListReq) -> ServiceResultPage<NoticeResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        Notice::select_sys_notice_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<NoticeResp>>(), x.total))?
    }

    pub async fn query_sys_notice_request(_rb: &RBatis, _item: QueryNoticeListReq) -> ServiceResult {
        let body = reqwest::get("http://dev.muche365.com/enter/lutong/order-request").await.unwrap().text().await.unwrap();

        info!("{}", body);

        ok()
    }
}

use crate::common::error::{AppError, ServiceResult, ServiceResultPage};
use crate::common::result::{ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_post_dao::SysPostDao;
use crate::dao::system::sys_user_post_dao;
use crate::model::system::sys_post_model::Post;
use crate::vo::system::sys_post_vo::{DeletePostReq, PostReq, PostResp, QueryPostDetailReq, QueryPostListReq, UpdatePostStatusReq};
use axum::Json;
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysPostService;

impl SysPostService {
    pub async fn add_sys_post(rb: &RBatis, mut item: PostReq) -> ServiceResult<String> {
        if Post::select_by_name(rb, &item.post_name).await?.is_some() {
            return Err(AppError::BusinessError("岗位名称已存在"));
        }

        if Post::select_by_code(rb, &item.post_code).await?.is_some() {
            return Err(AppError::BusinessError("岗位编码已存在"));
        }

        item.id = None;
        Post::insert(rb, &Post::from(item)).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_post(rb: &RBatis, item: DeletePostReq) -> ServiceResult<String> {
        let ids = item.ids.clone();
        for id in ids {
            if sys_user_post_dao::count_user_post_by_id(rb, id).await? > 0 {
                return Err(AppError::BusinessError("已分配,不能删除"));
            }
        }

        Post::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_post(rb: &RBatis, item: PostReq) -> ServiceResult<String> {
        let id = item.id;

        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Post::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("岗位不存在"));
        }

        if let Some(x) = Post::select_by_name(rb, &item.post_name).await? {
            if x.id != id {
                return Err(AppError::BusinessError("岗位名称已存在"));
            }
        }

        if let Some(x) = Post::select_by_code(rb, &item.post_code).await? {
            if x.id != id {
                return Err(AppError::BusinessError("岗位编码已存在"));
            }
        }

        Post::update_by_map(rb, &Post::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_post_status(rb: &RBatis, item: UpdatePostStatusReq) -> ServiceResult<String> {
        SysPostDao::update_status(rb, &item.ids, item.status).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_post_detail(rb: &RBatis, item: QueryPostDetailReq) -> ServiceResult<PostResp> {
        Post::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("岗位不存在")),
            |x| {
                let data: PostResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_post_list(rb: &RBatis, item: QueryPostListReq) -> ServiceResultPage<PostResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        Post::select_post_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<PostResp>>(), x.total))?
    }
}

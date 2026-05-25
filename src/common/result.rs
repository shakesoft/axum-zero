use crate::common::error::{AppResult};
use axum::Json;
use rbatis::rbdc::DateTime;
use serde::Serialize;
use std::fmt::Debug;
use utoipa::{ToSchema};

const SUCCESS_CODE:i32 = 0;
const SUCCESS_MSG:&str ="操作成功";
const DATETIME_FORMAT:&str = "YYYY-MM-DD hh:mm:ss";
const EMPTY_STRING:&str = "";

pub type EmptyResponse = BaseResponse<()>;

#[derive(Serialize, Debug, Clone,ToSchema)]
pub struct BaseResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone,ToSchema)]
pub struct PageResponse<T> {
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub data: Option<T>,
}

pub fn ok() -> AppResult<Json<EmptyResponse>> {
    Ok(Json(EmptyResponse {
        msg: SUCCESS_MSG.to_string(),
        code: SUCCESS_CODE,
        data: Some(()),
    }))
}

pub fn ok_result() -> AppResult<Json<BaseResponse<String>>> {
    ok_result_msg(SUCCESS_MSG)
}

pub fn ok_result_msg(msg: &str) -> AppResult<Json<BaseResponse<String>>> {
    Ok(Json(BaseResponse {
        msg: msg.to_string(),
        code: SUCCESS_CODE,
        data: None,
    }))
}

pub fn ok_result_data<T>(data: T) -> AppResult<Json<BaseResponse<T>>> {
    Ok(Json(BaseResponse {
        msg: SUCCESS_MSG.to_string(),
        code: SUCCESS_CODE,
        data: Some(data),
    }))
}

pub fn ok_result_page<T>(data: T, total: u64) -> AppResult<Json<PageResponse<T>>> {
    Ok(Json(PageResponse {
        msg: SUCCESS_MSG.to_string(),
        code: SUCCESS_CODE,
        data: Some(data),
        total,
    }))
}

pub fn serialize_datetime<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(datetime) => {
            let formatted = datetime.format(DATETIME_FORMAT);
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_str(EMPTY_STRING),
    }
}


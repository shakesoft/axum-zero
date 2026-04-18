use crate::common::result::{AppResult, BaseResponse};
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use redis::RedisError;
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;
const DEFAULT_ERROR_MSG:&str="服务器发生内部异常，请稍后再试";

#[derive(Serialize)]
struct ValidationErrorItem {
    message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidationErrorData {
    validation_errors: Vec<ValidationErrorItem>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    
    #[error("认证错误：{0}")]
    JwtTokenError(String),

    #[error("授权错误：{0}")]
    AuthorizationError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("Redis错误: {0}")]
    RedisError(#[from] RedisError),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),

    #[error("验证异常")]
    ValidationError(Vec<String>),

    #[error("内部异常: {0}")]
    InternalError(&'static str),
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let errors = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|error| {
                    if let Some(message) = &error.message {
                        message.clone().into_owned()
                    } else {
                        "Invalid value".to_string()
                    }
                })
            })
            .collect::<Vec<String>>();
        AppError::ValidationError(errors)
    }
}

impl From<JsonRejection> for AppError {
    fn from(error: JsonRejection) -> Self {
        AppError::ValidationError(vec![error.body_text()])
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let app_code = self.app_code();
        let message = self.response_message();

        match self {
            AppError::ValidationError(messages) => {
                let response = BaseResponse {
                    msg: message,
                    code: app_code,
                    data: Some(ValidationErrorData {
                        validation_errors: messages
                            .into_iter()
                            .map(|message| ValidationErrorItem { message })
                            .collect(),
                    }),
                };
                (status_code, Json(response)).into_response()
            },
            _ => {
                let response = BaseResponse {
                    msg: message,
                    code: app_code,
                    data: Some(()),
                };
                (status_code, Json(response)).into_response()
            },
        }
    }
}


impl AppError {
    fn app_code(&self) -> i32 {
        match self {
            AppError::DiskCacheRead { .. } => 9003,
            AppError::JwtTokenError(_) => 3001,
            AppError::AuthorizationError(_) => 4001,
            AppError::DbError(_) => 9001,
            AppError::RedisError(_) => 9002,
            AppError::BusinessError(_) => 5001,
            AppError::ValidationError(_) => 6001,
            AppError::InternalError(_) => 8001,
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::JwtTokenError(_) => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::BusinessError(_)=> StatusCode::FORBIDDEN,
            AppError::AuthorizationError(_) => StatusCode::UNAUTHORIZED,
            AppError::DbError(_)
            | AppError::RedisError(_)
            | AppError::DiskCacheRead { .. }
            | AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn response_message(&self) -> String {
        match self {
            AppError::ValidationError(_) => "参数校验失败".to_string(),
            _ => self.to_string(),
        }
    }

    pub fn default() -> AppError {
        AppError::InternalError(DEFAULT_ERROR_MSG)
    }
    pub fn interrupt() -> AppResult<Json<BaseResponse<()>>> {
        Err(AppError::default())
    }
}

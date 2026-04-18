use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;
use crate::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http, response, Json};
use redis::{Client};
use std::sync::Arc;
use log::info;
use crate::utils::jwt_util;
use crate::service::system::sys_user_service::SysUserService;
use crate::vo::system::sys_user_vo::UserSession;

#[function_name::named]
pub async fn auth(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Result<response::Response, StatusCode> {
    let path = req.uri().path();
    info!("{function_name}:request_uri {uri:?}",function_name = function_name!(),uri = path);
    let ignore_paths = vec![
        "/system/user/login",
        "/system/user/register",
        "/system/user/logout",
    ];
    if ignore_paths.iter().any(|ignore_path| path.starts_with(ignore_path)) {
        return Ok(next.run(req).await);
    }

    let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(|header| header.to_str().ok());
    if auth_header.is_none() {
        return Ok(AppError::JwtTokenError("用户未认证，请求头缺少[Authorization]".to_string()).into_response());
    }

    let authorization = auth_header.unwrap();
    let token = authorization.to_string().replace("Bearer ", "");
    let jwt_token_error = JwtToken::verify(jwt_util::JWT_SECRET, &token);
    let jwt_token = match jwt_token_error {
        Ok(data) => data,
        Err(err) => {
            let er = match err {
                AppError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };
            return Ok(AppError::JwtTokenError(er).into_response());
        }
    };

    match validate_and_get_user_info(&state.redis, jwt_token.id).await {
        Ok((user_id, permissions,roles, redis_token, is_admin)) => {
            if redis_token != token {
                return Ok(AppError::JwtTokenError("无效的JwtToken".to_string()).into_response());
            }

            let has_permission = |permissions: &[String], path: &str| -> bool {
                permissions.iter().any(|permission| {
                    permission == path
                })
            };

            if is_admin || has_permission(&permissions,path) {
                req.headers_mut().insert("user_id", user_id.to_string().parse().unwrap());//存储用户Id
                // req.extensions_mut().insert(permissions);//存储用户功能权限
                req.extensions_mut().insert(UserSession {//存储用户会话
                    user_id,
                    permissions,
                    roles,
                    is_admin,
                });//存储用户功能权限
                Ok(next.run(req).await)
            } else {
                Ok(AppError::AuthorizationError(format!("用户未授权访问Url:{}", path)).into_response())
            }
        }
        Err(e) => {
            match e {
                AppError::BusinessError(msg) if msg == "用户未登录" => {
                    Ok(AppError::JwtTokenError(msg.to_string()).into_response())
                }
                _ => Ok(e.into_response()),
            }
        }
    }
}

async fn validate_and_get_user_info(redis_client: &Client, user_id: i64) -> Result<(i64, Vec<String>, Vec<String>, String, bool), AppError> {
    let mut conn = redis_client.get_connection().map_err(AppError::RedisError)?;

    // Fetch session fields from Redis via service method, which handles key existence and field retrieval
    let (permissions_str,roles_str, token, is_admin) = SysUserService::fetch_session_info(&mut conn, user_id).await?;

    // helper to parse comma-separated values into trimmed non-empty strings
    let parse_csv = |s: &str| -> Vec<String> {
        if s.trim().is_empty() {
            Vec::new()
        } else {
            s.split(',')
                .map(|p| p.trim())
                .filter(|p| !p.is_empty())
                .map(|p| p.to_string())
                .collect()
        }
    };

    let vec_roles = parse_csv(&roles_str);
    let vec_permissions = parse_csv(&permissions_str);

    // normalize permissions once (strip leading "/api" if present) and collect into a Vec
    let vec_permissions: Vec<String> = vec_permissions
        .iter()
        .map(|p| p.strip_prefix("/api").unwrap_or(p).to_string())
        .collect();
    // println!("{user_id},{vec_permissions:?},{vec_roles:?}");
    Ok((user_id, vec_permissions,vec_roles, token, is_admin))
}

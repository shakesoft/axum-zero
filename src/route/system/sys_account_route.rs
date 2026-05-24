use crate::handler::system::sys_user_handler;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

/*
 *构建用户信息路由
 *author：罗京生
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_account_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/system/user/login", post(sys_user_handler::login))
    //记得在main.rs中添加路由build_sys_account_route()
}

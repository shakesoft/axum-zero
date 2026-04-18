use crate::common::error::AppError;
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_post_model::UserPost;
use crate::model::system::sys_user_role_model::{is_admin, UserRole};
use crate::dao::system::sys_user_dao::SysUserDao;
use crate::utils::jwt_util::JwtToken;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::vo::system::sys_dept_vo::DeptResp;
use crate::vo::system::sys_role_vo::RoleResp;
use crate::vo::system::sys_user_vo::*;
use crate::AppState;
use axum::extract::{State, ConnectInfo};
use std::net::SocketAddr;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Local;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;
use rbs::value;
use redis::Commands;
use std::collections::{HashMap, HashSet};
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use aspect_macros::aspect;
use aspect_std::TimingAspect;
use aspect_std::LoggingAspect;
use aspect_std::RateLimitAspect;
use axum_valid::Valid;
use crate::aop::aspects::logger::Logger;
use crate::aop::aspects::timer::Timer;
use crate::service::system::sys_login_log_service::SysLoginLogService;
use crate::service::system::sys_user_service::SysUserService;
use crate::utils::{jwt_util, time_util};
/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn add_sys_user(State(state): State<Arc<AppState>>, Json(mut item): Json<UserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    if User::select_by_user_name(rb, &item.user_name).await?.is_some() {
        return Err(AppError::BusinessError("登录账号已存在"));
    }

    if User::select_by_mobile(rb, &item.mobile).await?.is_some() {
        return Err(AppError::BusinessError("手机号码已存在"));
    }

    if User::select_by_email(rb, &item.email).await?.is_some() {
        return Err(AppError::BusinessError("邮箱账号已存在"));
    }

    let post_ids = item.post_ids.clone();
    item.id = None;
    let id = User::insert(rb, &User::from(item)).await?.last_insert_id;

    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in post_ids {
        user_post_list.push(UserPost { user_id: id.i64(), post_id })
    }

    UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await.map(|_| ok_result())?
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn delete_sys_user(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<DeleteUserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    let ids = item.ids.clone();
    if ids.contains(&user_id) {
        return Err(AppError::BusinessError("当前用户不能删除"));
    }

    for id in ids.clone() {
        let key = format!("axum:admin:user:info:{}", id);
        let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

        if is_admin {
            return Err(AppError::BusinessError("不允许操作超级管理员用户"));
        }
    }

    UserRole::delete_by_map(rb, value! {"user_id": &ids}).await?;
    UserPost::delete_by_map(rb, value! {"user_id": &ids}).await?;
    User::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user(State(state): State<Arc<AppState>>, Json(item): Json<UserReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let mut conn = state.redis.get_connection()?;
    let id = item.id;
    if item.id.is_none() {
        return Err(AppError::BusinessError("主键不能为空"));
    }

    let key = format!("axum:admin:user:info:{}", id.unwrap_or_default());
    let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    let user = match User::select_by_id(rb, id.unwrap_or_default()).await? {
        None => return Err(AppError::BusinessError("用户不存在")),
        Some(x) => x,
    };

    if let Some(x) = User::select_by_user_name(rb, &item.user_name).await? {
        if x.id != id {
            return Err(AppError::BusinessError("登录账号已存在"));
        }
    }

    if let Some(x) = User::select_by_mobile(rb, &item.mobile).await? {
        if x.id != id {
            return Err(AppError::BusinessError("手机号码已存在"));
        }
    }

    if let Some(x) = User::select_by_email(rb, &item.email).await? {
        if x.id != id {
            return Err(AppError::BusinessError("邮箱账号已存在"));
        }
    }

    let post_ids = item.post_ids.clone();
    let mut user_post_list: Vec<UserPost> = Vec::new();
    for post_id in post_ids {
        user_post_list.push(UserPost {
            user_id: user.id.unwrap_or_default(),
            post_id,
        })
    }

    UserPost::delete_by_map(rb, value! {"user_id": &item.id}).await?;
    UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await?;
    User::update_by_map(rb, &User::from(item), value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let ids = item.ids.clone();
    let mut conn = state.redis.get_connection()?;

    for id in ids {
        let key = format!("axum:admin:user:info:{}", id);
        let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

        if is_admin {
            return Err(AppError::BusinessError("不允许操作超级管理员用户"));
        }
    }

    SysUserDao::update_status(rb, &item.ids, item.status).await.map(|_| ok_result())?
}

/*
 *重置用户密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn reset_sys_user_password(State(state): State<Arc<AppState>>, Json(item): Json<ResetUserPwdReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());

    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let key = format!("axum:admin:user:info:{}", item.id.clone());
    let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    let sys_user_result = User::select_by_id(rb, item.id).await?;

    match sys_user_result {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(x) => {
            let mut user = x;
            user.password = item.password;
            User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result())?
        }
    }
}

/*
 *用户修改自己的密码
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn update_sys_user_password(headers: HeaderMap, State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());

    let rb = &state.batis;

    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

    match User::select_by_id(rb, user_id).await? {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(x) => {
            let mut user = x;
            if user.password != item.pwd {
                return Err(AppError::BusinessError("旧密码不正确"));
            }
            user.password = item.re_pwd;
            User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result())?
        }
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_sys_user_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let mut x = match User::select_by_id(rb, item.id).await? {
        None => return Err(AppError::BusinessError("用户不存在")),
        Some(user) => {
            let a: UserResp = user.into();
            a
        }
    };

    let dept = match Dept::select_by_id(rb, &x.dept_id).await? {
        None => return Err(AppError::BusinessError("部门不存在")),

        Some(y) => {
            let a: DeptResp = y.into();
            Some(a)
        }
    };

    let post_ids = UserPost::select_by_map(rb, value! {"user_id": item.id}).await?.iter().map(|x| x.post_id).collect::<Vec<i64>>();

    x.dept_info = dept;
    x.post_ids = Some(post_ids);

    ok_result_data(x)
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */

#[function_name::named]
// #[aspect(Timer)]
// #[aspect(Logger)]
#[aspect(LoggingAspect::new())]
pub async fn query_sys_user_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;
    let num = add(1, 2).await;
    let page = &PageRequest::new(item.page_no, item.page_size);

    User::select_sys_user_list(rb, page, &item)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<UserResp>>(), x.total))?
}

// #[aspect(Logger)]
// #[aspect(Timer)]
async fn add2(num1:i32,num2:i32)->i32{
    num1+num2
}

// #[aspect(Logger)]
// #[aspect(Logger)]
async fn add(num1: i32, num2: i32) -> i32 {
    let res = add2(num1,num2).await;
    num1 + num2 +res
}

/*
 *用户登录
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
#[aspect(Timer)]
pub async fn login(
    headers: HeaderMap,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
    Valid(Json(item)): Valid<Json<UserLoginReq>>,
) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_agent = headers.get("User-Agent").unwrap().to_str().unwrap();
    info!("user agent: {:?}", user_agent);
    let agent = UserAgentUtil::new(user_agent);

    let user_result = User::select_by_mobile(rb, &item.mobile).await?;
    info!("query user by mobile: {:?}", user_result);

    match user_result {
        None => {
            SysLoginLogService::add_login_log(rb, item.mobile, 0, "用户名或密码不正确", agent).await;
            Err(AppError::BusinessError("用户名或密码不正确"))
        }
        Some(mut user) => {
            if (&user.password.ne(&item.password)).to_owned() {
                SysLoginLogService::add_login_log(rb, item.mobile, 0, "用户名或密码不正确", agent).await;
                Err(AppError::BusinessError("用户名或密码不正确"))
            }
            else {
                // determine client IP: prefer X-Forwarded-For (first IP), then X-Real-IP, otherwise use the TCP peer address
                let client_ip = if let Some(v) = headers.get("X-Forwarded-For") {
                    v.to_str().ok().map(|s| s.split(',').next().unwrap_or("").trim().to_string()).unwrap_or_default()
                } else if let Some(v) = headers.get("X-Real-IP") {
                    v.to_str().ok().map(|s| s.to_string()).unwrap_or_default()
                } else {
                    remote_addr.ip().to_string()
                };

                // create token, cache session, update user，logging
                let resp = SysUserService::user_login_successful(
                    rb,
                    &mut conn,
                    &mut user,
                    item.mobile,
                    agent,
                    client_ip,
                ).await?;
                ok_result_data(resp)
            }
        }
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[function_name::named]
pub async fn query_user_role(State(state): State<Arc<AppState>>, Json(item): Json<QueryUserRoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;

    let role_list = Role::select_all(rb).await.map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<RoleResp>>())?;
    let sys_role_list = role_list.clone();
    let mut user_role_ids = role_list.into_iter().map(|x| x.id.unwrap_or_default()).collect::<Vec<i64>>();

    if item.user_id != 1 {
        let vec1 = UserRole::select_by_map(rb, value! {"user_id": item.user_id}).await?;
        user_role_ids = vec1.into_iter().map(|x| x.id.unwrap_or_default()).collect::<Vec<i64>>();
    }

    ok_result_data(QueryUserRoleResp { sys_role_list, user_role_ids })
}

// 更新用户角色
#[function_name::named]
pub async fn update_user_role(State(state): State<Arc<AppState>>, Json(item): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}",function_name = function_name!());
    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    let user_id = item.user_id;
    let role_ids = &item.role_ids;
    let len = item.role_ids.len();

    let key = format!("axum:admin:user:info:{}", user_id);
    let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

    if is_admin {
        return Err(AppError::BusinessError("不允许操作超级管理员用户"));
    }

    UserRole::delete_by_map(rb, value! {"user_id": user_id}).await?;

    let mut list: Vec<UserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        list.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    UserRole::insert_batch(rb, &list, len as u64).await.map(|_| ok_result())?
}

// 查询用户菜单
#[function_name::named]
pub async fn query_user_menu(headers: HeaderMap, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();
    info!("{function_name}:{user_id:?}",function_name = function_name!());

    let rb = &state.batis;
    let mut conn = state.redis.get_connection()?;

    //根据id查询用户
    match User::select_by_id(rb, user_id).await? {
        None => Err(AppError::BusinessError("用户不存在")),
        Some(user) => {
            let key = format!("axum:admin:user:info:{}", user_id);
            let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

            // 从服务层获取菜单列表（服务层封装了 DB 查询）
            let sys_menu_list: Vec<Menu> = SysUserDao::fetch_user_menus(rb, user_id, is_admin).await?;

            let mut sys_menu: Vec<MenuList> = Vec::new();
            let mut btn_menu: Vec<String> = Vec::new();
            let mut sys_menu_ids: HashSet<i64> = HashSet::new();

            for x in sys_menu_list {
                if x.visible == 0 {
                    continue;
                }
                if x.menu_type != 3 {
                    sys_menu_ids.insert(x.id.unwrap_or_default().clone());
                    sys_menu_ids.insert(x.parent_id.unwrap_or_default().clone());
                }

                if x.api_url.clone().unwrap_or_default().len() > 0 {
                    btn_menu.push(x.api_url.unwrap_or_default());
                }
            }

            let mut menu_ids = Vec::new();
            for id in sys_menu_ids {
                menu_ids.push(id)
            }
            let vec1 = Menu::select_by_ids(rb, &menu_ids).await?;
            for menu in vec1 {
                sys_menu.push(MenuList {
                    id: menu.id,
                    parent_id: menu.parent_id,
                    name: menu.menu_name,
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.as_ref().map_or_else(|| "".to_string(), |url| url.to_string()),
                    menu_type: menu.menu_type,
                    path: menu.menu_url.unwrap_or_default(),
                });
            }


            let resp = QueryUserMenuResp {
                sys_menu,
                btn_menu,
                avatar: user.avatar,
                name: user.user_name,
            };

            ok_result_data(resp)
        }
    }
}

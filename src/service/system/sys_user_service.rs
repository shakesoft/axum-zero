use crate::common::error::{AppError, AppResult, ServiceResult, ServiceResultPage};
use crate::common::result::{ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_user_dao::SysUserDao;
use crate::model::system::sys_dept_model::Dept;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_post_model::UserPost;
use crate::model::system::sys_user_role_model::UserRole;
use crate::service::system::sys_login_log_service::SysLoginLogService;
use crate::utils::jwt_util::JwtToken;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::utils::{jwt_util, time_util};
use crate::vo::system::sys_dept_vo::DeptResp;
use crate::vo::system::sys_role_vo::RoleResp;
use crate::vo::system::sys_user_vo::*;
use axum::http::HeaderMap;
use chrono::Local;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;
use rbs::value;
use redis::Commands;
use std::collections::HashSet;
use std::net::SocketAddr;

pub struct SysUserService;

impl SysUserService {
    pub async fn add_sys_user(rb: &RBatis, mut item: UserReq) -> ServiceResult<String> {
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

        UserPost::insert_batch(rb, &user_post_list, user_post_list.len() as u64).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_user(rb: &RBatis, redis: &redis::Client, headers: HeaderMap, item: DeleteUserReq) -> ServiceResult<String> {
        let mut conn = redis.get_connection()?;
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
        User::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_user(rb: &RBatis, redis: &redis::Client, item: UserReq) -> ServiceResult<String> {
        let mut conn = redis.get_connection()?;
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
        User::update_by_map(rb, &User::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_user_status(rb: &RBatis, redis: &redis::Client, item: UpdateUserStatusReq) -> ServiceResult<String> {
        let ids = item.ids.clone();
        let mut conn = redis.get_connection()?;

        for id in ids {
            let key = format!("axum:admin:user:info:{}", id);
            let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

            if is_admin {
                return Err(AppError::BusinessError("不允许操作超级管理员用户"));
            }
        }

        SysUserDao::update_status(rb, &item.ids, item.status).await.map(|_| ok_result_empty())?
    }

    pub async fn reset_sys_user_password(rb: &RBatis, redis: &redis::Client, item: ResetUserPwdReq) -> ServiceResult<String> {
        let mut conn = redis.get_connection()?;

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
                User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result_empty())?
            }
        }
    }

    pub async fn update_sys_user_password(rb: &RBatis, headers: HeaderMap, item: UpdateUserPwdReq) -> ServiceResult<String> {
        let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();

        match User::select_by_id(rb, user_id).await? {
            None => Err(AppError::BusinessError("用户不存在")),
            Some(x) => {
                let mut user = x;
                if user.password != item.pwd {
                    return Err(AppError::BusinessError("旧密码不正确"));
                }
                user.password = item.re_pwd;
                User::update_by_map(rb, &user, value! {"id": &user.id}).await.map(|_| ok_result_empty())?
            }
        }
    }

    pub async fn query_sys_user_detail(rb: &RBatis, item: QueryUserDetailReq) -> ServiceResult<UserResp> {
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

    pub async fn query_sys_user_list(rb: &RBatis, item: QueryUserListReq) -> ServiceResultPage<UserResp> {
        let _num = add(1, 2).await;
        let page = &PageRequest::new(item.page_no, item.page_size);

        User::select_sys_user_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<UserResp>>(), x.total))?
    }

    pub async fn login(rb: &RBatis, redis: &redis::Client, headers: HeaderMap, remote_addr: SocketAddr, item: UserLoginReq) -> ServiceResult<UserLoginResp> {
        let mut conn = redis.get_connection()?;

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
                } else {
                    let client_ip = if let Some(v) = headers.get("X-Forwarded-For") {
                        v.to_str().ok().map(|s| s.split(',').next().unwrap_or("").trim().to_string()).unwrap_or_default()
                    } else if let Some(v) = headers.get("X-Real-IP") {
                        v.to_str().ok().map(|s| s.to_string()).unwrap_or_default()
                    } else {
                        remote_addr.ip().to_string()
                    };

                    let resp = SysUserService::user_login_successful(rb, &mut conn, &mut user, item.mobile, agent, client_ip).await?;
                    ok_result_data(resp)
                }
            }
        }
    }

    pub async fn query_user_role(rb: &RBatis, item: QueryUserRoleReq) -> ServiceResult<QueryUserRoleResp> {
        let role_list = Role::select_all(rb).await.map(|x| x.into_iter().map(|x| x.into()).collect::<Vec<RoleResp>>())?;
        let sys_role_list = role_list.clone();
        let mut user_role_ids = role_list.into_iter().map(|x| x.id.unwrap_or_default()).collect::<Vec<i64>>();

        if item.user_id != 1 {
            let vec1 = UserRole::select_by_map(rb, value! {"user_id": item.user_id}).await?;
            user_role_ids = vec1.into_iter().map(|x| x.id.unwrap_or_default()).collect::<Vec<i64>>();
        }

        ok_result_data(QueryUserRoleResp { sys_role_list, user_role_ids })
    }

    pub async fn update_user_role(rb: &RBatis, redis: &redis::Client, item: UpdateUserRoleReq) -> ServiceResult<String> {
        let mut conn = redis.get_connection()?;

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

        UserRole::insert_batch(rb, &list, len as u64).await.map(|_| ok_result_empty())?
    }

    pub async fn query_user_menu(rb: &RBatis, redis: &redis::Client, headers: HeaderMap) -> ServiceResult<QueryUserMenuResp> {
        let user_id = headers.get("user_id").unwrap().to_str().unwrap().parse::<i64>().unwrap();
        let mut conn = redis.get_connection()?;

        match User::select_by_id(rb, user_id).await? {
            None => Err(AppError::BusinessError("用户不存在")),
            Some(user) => {
                let key = format!("axum:admin:user:info:{}", user_id);
                let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();

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

    /// Called when a user successfully authenticates.
    ///
    /// This will:
    /// - create a JWT token
    /// - store permission and session info into redis
    /// - write a successful login log
    /// - update the user's last login info in the database
    pub async fn user_login_successful(rb: &RBatis, conn: &mut redis::Connection, user: &mut User, mobile: String, agent: UserAgentUtil, client_ip: String) -> Result<UserLoginResp, AppError> {
        let user_id = user.id.unwrap();

        //获取用户权限列表
        let (btn_menu, is_admin) = SysUserDao::query_btn_menu(rb, &user_id).await;
        if btn_menu.len() == 0 {
            SysLoginLogService::add_login_log(rb, mobile, 0, "用户没有分配角色或者菜单,不能登录", agent).await;
            return Err(AppError::BusinessError("用户没有分配角色或者菜单,不能登录"));
        }

        //获取用户角色列表
        let vec_user_role = UserRole::select_by_map(rb, value! {"user_id": &user_id}).await?;
        let vec_role_id = vec_user_role.into_iter().map(|x| x.role_id.to_string()).collect::<Vec<String>>();

        // generate token
        let jwt = JwtToken::new(user_id, &user.user_name);
        let expires_at = jwt.get_exp();
        let expires_time = time_util::timestamp_to_local(expires_at as i64);
        let token = jwt.create_token(jwt_util::JWT_SECRET)?;

        // persist session info to redis
        let key = format!("axum:admin:user:info:{:?}", user_id);
        conn.hset::<_, _, _, ()>(&key, "permissions", &btn_menu.join(","))?; // permissions
        conn.hset::<_, _, _, ()>(&key, "roles", &vec_role_id.join(","))?; // role ids
        conn.hset::<_, _, _, ()>(&key, "user_name", &user.user_name)?; // user name
        conn.hset::<_, _, _, ()>(&key, "is_admin", is_admin)?; // is super
        conn.hset::<_, _, _, ()>(&key, "token", &token)?; // token
        conn.hset::<_, _, _, ()>(&key, "last_login", Local::now().format("%Y-%m-%d %H:%M:%S").to_string())?;
        conn.hset::<_, _, _, ()>(&key, "expires_at", expires_time.unwrap_or_default().format("%Y-%m-%d %H:%M:%S").to_string())?;

        // record login log
        SysLoginLogService::add_login_log(rb, mobile, 1, "登录成功", agent.clone()).await;

        // update user last login info
        user.login_ip = client_ip;
        user.login_date = Some(DateTime::now());
        user.login_browser = agent.browser;
        user.login_os = agent.os;

        // persist changes to DB
        User::update_by_map(rb, &user, value! {"id": &user_id}).await?;

        Ok(UserLoginResp { token, expires_at })
    }

    /// Fetch raw session fields from redis for a user
    /// Returns (permissions_str, roles_str, token, is_admin) or Err(AppError)
    pub async fn fetch_session_info(conn: &mut redis::Connection, user_id: i64) -> Result<(String, String, String, bool), AppError> {
        let key = format!("axum:admin:user:info:{}", user_id);

        // check key existence first
        let exists: bool = conn.exists(&key).map_err(AppError::RedisError)?;
        if !exists {
            return Err(AppError::BusinessError("用户没有登录"));
        }

        let token: String = conn.hget(&key, "token").map_err(AppError::RedisError)?;
        let is_admin: bool = conn.hget(&key, "is_admin").unwrap_or_default();
        let permissions_str: String = conn.hget(&key, "permissions").unwrap_or_else(|_| "".to_string());
        let roles_str: String = conn.hget(&key, "roles").unwrap_or_else(|_| "".to_string());
        let expires_at: String = conn.hget(&key, "expires_at").unwrap_or_else(|_| "".to_string());
        //info!("Fetched session info for user_id {}: token={}, is_admin={}, permissions={}, roles={}, expires_at={}", user_id, token, is_admin, permissions_str,roles_str, expires_at);
        Ok((permissions_str, roles_str, token, is_admin))
    }
}

async fn add2(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

async fn add(num1: i32, num2: i32) -> i32 {
    let res = add2(num1, num2).await;
    num1 + num2 + res
}

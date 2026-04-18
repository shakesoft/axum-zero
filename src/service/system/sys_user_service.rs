use crate::common::error::AppError;
use crate::model::system::sys_user_model::User;
use crate::service::system::sys_login_log_service::SysLoginLogService;
use crate::utils::{jwt_util, time_util};
use crate::utils::jwt_util::JwtToken;
use crate::vo::system::sys_user_vo::UserLoginResp;
use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use chrono::Local;
use log::info;
use rbs::value;
use redis::Commands;
use crate::dao::system::sys_user_dao::SysUserDao;
use crate::model::system::sys_user_role_model::UserRole;
use crate::utils::user_agent_util::UserAgentUtil;

pub struct SysUserService;

impl SysUserService {
	/// Called when a user successfully authenticates.
	///
	/// This will:
	/// - create a JWT token
	/// - store permission and session info into redis
	/// - write a successful login log
	/// - update the user's last login info in the database
	pub async fn user_login_successful(
		rb: &RBatis,
		conn: &mut redis::Connection,
		user: &mut User,
		mobile: String,
		agent: UserAgentUtil,
		client_ip: String,
	) -> Result<UserLoginResp, AppError> {
		let user_id =user.id.unwrap();

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
	pub async fn fetch_session_info(
		conn: &mut redis::Connection,
		user_id: i64,
	) -> Result<(String, String, String, bool), AppError> {
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
		info!("Fetched session info for user_id {}: token={}, is_admin={}, permissions={}, roles={}, expires_at={}", user_id, token, is_admin, permissions_str,roles_str, expires_at);
		Ok((permissions_str,roles_str, token, is_admin))
	}
}
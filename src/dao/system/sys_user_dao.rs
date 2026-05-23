use rbatis::RBatis;
use rbs::{value, Error};
use std::collections::HashMap;
use rbatis::executor::Executor;
use crate::dao::system::sys_user_role_dao;
use crate::model::system::sys_user_model::User;
use crate::vo::system::sys_user_vo::QueryUserListReq;

pub struct SysUserDao;

impl SysUserDao {
    // 封装更新用户状态的数据库操作（批量）
    pub async fn update_status(rb: &RBatis, ids: &Vec<i64>, status: i8) -> rbatis::Result<()> {
        let update_sql = format!("update sys_user set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }

    // 封装查询用户按钮权限和是否为超级管理员的数据库操作
    // 保持签名与原函数兼容：接收 RBatis 的 owned 值和用户 id
    pub async fn query_btn_menu(rb: &RBatis, id: &i64) -> (Vec<String>, bool) {
        // 使用模型层提供的 is_admin 函数判断是否是超级管理员
        let count = sys_user_role_dao::is_admin(&rb, id).await.unwrap_or_default();
        let mut btn_menu: Vec<String> = Vec::new();
        if count == 1 {
            // 超级管理员：返回所有菜单的 api_url
            let data = crate::model::system::sys_menu_model::Menu::select_all(rb).await;
            for x in data.unwrap_or_default() {
                if let Some(a) = x.api_url {
                    if a != "" {
                        btn_menu.push(a);
                    }
                }
            }
            (btn_menu, true)
        } else {
            // 普通用户：通过查询聚合权限
            let sql = "select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
            let btn_menu_map: Vec<HashMap<String, String>> = rb.query_decode(sql, vec![value!(id)]).await.unwrap_or_default();
            for x in btn_menu_map {
                if let Some(a) = x.get("api_url") {
                    if a.to_string() != "" {
                        btn_menu.push(a.to_string());
                    }
                }
            }
            (btn_menu, false)
        }
    }

    // 新增：根据用户是否为超级管理员从数据库获取菜单列表
    pub async fn fetch_user_menus(rb: &RBatis, user_id: i64, is_admin: bool) -> rbatis::Result<Vec<crate::model::system::sys_menu_model::Menu>> {
        if is_admin {
            crate::model::system::sys_menu_model::Menu::select_all(rb).await
        } else {
            let sql = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
            rb.query_decode(sql, vec![value!(user_id)]).await
        }
    }
}



/*
 *用户信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(User {}, "sys_user");

/*
 *根据id查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_id(id:i64) -> Option => "`where id = #{id} limit 1`"}, "sys_user");

/*
 *根据mobile查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_mobile(mobile:&str) -> Option => "`where mobile = #{mobile} limit 1`"},"sys_user");

/*
 *根据user_name查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_user_name(user_name:&str) -> Option => "`where user_name = #{user_name} limit 1`"}, "sys_user");

/*
 *根据email查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(User{select_by_email(email:&str) -> Option => "`where email = #{email} limit 1`"}, "sys_user");

/*
 *分页查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(User{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_user");

/*
 *根据条件分页查询用户信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(User{select_sys_user_list(req:&QueryUserListReq) =>"
      where 1=1
      if req.mobile != null && req.mobile != '':
       ` and mobile like concat('%', #{req.mobile}, '%') `
     if req.userName != null && req.userName != '':
       ` and user_name like concat('%', #{req.userName}, '%') `
     if req.status != 2:
       ` and status = #{req.status} `
     if req.deptId != 0:
       ` and (dept_id = #{req.deptId} OR dept_id IN (SELECT id FROM sys_dept WHERE find_in_set(#{req.deptId}, ancestors))) `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_user");

/*
 *根据条件分页查询已配用户角色列表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[py_sql(
    "`select u.* from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            limit #{page_no},#{page_size}` "
)]
async fn select_allocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str, page_no: u64, page_size: u64) -> Result<Vec<User>, Error> {
    impled!()
}

/*
 * 描述：根据条件分页查询已配用户角色数量
 * author：刘飞华
 * date：2025/1/6 16:13
 */
#[py_sql(
    "`select count(1) from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and ur.role_id = #{role_id} `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `"
)]
async fn count_allocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str) -> Result<u64, Error> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色列表
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select u.* from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and (ur.role_id != #{role_id} or ur.role_id is null) `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `
            limit #{page_no},#{page_size}` "
)]
pub async fn select_unallocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str, page_no: u64, page_size: u64) -> rbatis::Result<Vec<User>> {
    impled!()
}

/*
 * 描述：根据条件分页查询未分配用户角色数量
 * author：刘飞华
 * date：2025/1/6 16:17
 */
#[py_sql(
    "`select count(1) from sys_user u left join sys_user_role ur on u.id = ur.user_id where u.del_flag = 1 and (ur.role_id != #{role_id} or ur.role_id is null) `
            if mobile != '':
                ` and u.mobile = #{mobile} `
            if user_name != '':
                ` and u.user_name = #{user_name} `"
)]
pub async fn count_unallocated_list(rb: &dyn Executor, role_id: i64, user_name: &str, mobile: &str) -> rbatis::Result<u64> {
    impled!()
}
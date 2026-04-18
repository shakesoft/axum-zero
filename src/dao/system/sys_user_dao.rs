use rbatis::RBatis;
use rbs::value;
use std::collections::HashMap;

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
        let count = crate::model::system::sys_user_role_model::is_admin(&rb, id).await.unwrap_or_default();
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

/*
 *菜单角色关联表基本操作
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
use std::collections::HashMap;
use rbatis::RBatis;
use crate::model::system::sys_role_menu_model::RoleMenu;

rbatis::crud!(RoleMenu {}, "sys_role_menu");

/*
 *根据角色id查询菜单ids
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[sql("select menu_id from sys_role_menu where role_id = ?")]
pub async fn query_menu_by_role(rb: &RBatis, role_id: i64) -> rbatis::Result<Vec<HashMap<String, i64>>> {
    impled!()
}

/*
 *查询菜单使用数量
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_role_menu where menu_id= ?")]
pub async fn select_count_menu_by_menu_id(rb: &RBatis, menu_id: &i64) -> rbatis::Result<i64> {
    impled!()
}

// author：罗京生
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
 *菜单角色关联表
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleMenu {
    pub id: Option<i64>,               //主键
    pub role_id: i64,                  //角色ID
    pub menu_id: i64,                  //菜单ID
    pub create_time: Option<DateTime>, //创建时间
}

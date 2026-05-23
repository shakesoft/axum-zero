// author：刘飞华
// createTime：2024/12/12 14:41:44

use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use serde::{Deserialize, Serialize};

/*
 *角色用户关联表
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<i64>,               //主键
    pub user_id: i64,                  //用户ID
    pub role_id: i64,                  //角色ID
    pub create_time: Option<DateTime>, //创建时间
}

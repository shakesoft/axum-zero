use crate::rbatis::rbatis_codegen::IntoSql;
use crate::vo::system::sys_menu_vo::{MenuReq, MenuResp};
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use rbs::value;
use serde::{Deserialize, Serialize};
use crate::model::system::sys_menu_model::Menu;

pub struct SysMenuDao;

impl SysMenuDao {
    // 公共服务函数：批量更新菜单状态
    pub async fn update_menu_status(rb: &RBatis, status: i8, ids: &Vec<i64>) -> rbatis::Result<()> {
        let update_sql = format!(
            "update sys_menu set status = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }
}


/*
 *菜单信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Menu {}, "sys_menu");


/*
 *根据id查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_menu");

/*
 *根据menu_name查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_menu_name(menu_name:&str) -> Option => "`where menu_name = #{menu_name} limit 1`"}, "sys_menu");

/*
 *根据menu_url查询菜单信息
 *author：刘飞华
 *date：2025/01/04 22:24:01
 */
impl_select!(Menu{select_by_menu_url(menu_url:&str) -> Option => "`where menu_url = #{menu_url} limit 1`"}, "sys_menu");

/*
 *根据ids查询菜单信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Menu{select_by_ids(ids:&[i64]) -> Vec => "`where status = 1 and id in ${ids.sql()} order by sort asc`"}, "sys_menu");

/*
 *查询菜单数量
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_menu where parent_id= ?")]
pub async fn select_count_menu_by_parent_id(rb: &RBatis, parent_id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2025/01/04 22:24:01
 */
impl_select!(Menu{select_menu_list() -> Vec => "`where menu_type != 3 and status = 1`"}, "sys_menu");
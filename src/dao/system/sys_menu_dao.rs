use rbatis::RBatis;
use rbs::value;

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
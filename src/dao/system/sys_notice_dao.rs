use rbatis::RBatis;
use rbs::value;
pub struct SysNoticeDao;

impl SysNoticeDao {
    // 封装更新通知公告状态的数据库操作
    pub async fn update_status(rb: &RBatis, ids: &Vec<i64>, status: i8) -> rbatis::Result<()> {
        let update_sql = format!("update sys_notice set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![rbs::Value::from(status)];
        param.extend(ids.iter().map(|&id| rbs::Value::from(id)));

        rb.exec(&update_sql, param).await.map(|_| ())
    }
}
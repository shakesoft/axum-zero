use rbatis::RBatis;
use rbs::value;

pub struct SysDictTypeDao;

impl SysDictTypeDao {
    // 公共服务函数：批量更新字典类型状态
    pub async fn update_dict_type_status(rb: &RBatis, status: i8, ids: &Vec<i64>) -> rbatis::Result<()> {
        let update_sql = format!(
            "update sys_dict_type set status = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }
}
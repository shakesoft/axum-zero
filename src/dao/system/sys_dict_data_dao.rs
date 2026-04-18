use rbatis::RBatis;
use rbs::value;

pub struct SysDictDataDao;

impl SysDictDataDao {

    /*
 *更新字典数据状态（批量）
 *author：自动提取并封装
 */
    pub async fn update_dict_data_status(rb: &RBatis, status: i8, ids: &Vec<i64>) -> rbatis::Result<()> {
        let update_sql = format!(
            "update sys_dict_data set status = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }

}
use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::QueryDictDataListReq;

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


/*
 *字典数据表基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(DictData {}, "sys_dict_data");


/*
 *根据id查询字典数据表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_dict_data");

/*
 *根据dict_type和dict_label查询字典数据表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_dict_label(dict_type:&str, dict_label:&str) -> Option => "`where dict_type = #{dict_type} and dict_label = #{dict_label}`"}, "sys_dict_data");

/*
 *根据dict_type和dict_value查询字典数据表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(DictData{select_by_dict_value(dict_type:&str, dict_value:&str) -> Option => "`where dict_type = #{dict_type} and dict_value = #{dict_value}`"}, "sys_dict_data");

/*
 *分页查询字典数据表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictData{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dict_data");

/*
 *根据条件分页查询字典数据表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictData{select_dict_data_list(req:&QueryDictDataListReq) =>"
    where 1=1
     if req.dictLabel != null && req.dictLabel != '':
      ` and dict_label like concat('%', #{req.dictLabel}, '%') `
     if req.dictType != null && req.dictType != '':
      ` and dict_type like concat('%', #{req.dictType}, '%') `
     if req.status != 2:
      ` and status = #{req.status} `
     if !sql.contains('count'):
      ` order by create_time desc"
},"sys_dict_data");

/*
 *同步修改字典类型
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
#[sql("update sys_dict_data set dict_type = ? where dict_type = ?")]
pub async fn update_dict_data_type(rb: &RBatis, new_dict_type: &str, old_dict_type: &str) -> Option<i64> {
    impled!()
}

/*
 *查询字典数据
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
#[sql("select count(1) from sys_dict_data where dict_type= ?")]
pub async fn count_dict_data_by_type(rb: &RBatis, dict_type: &str) -> rbatis::Result<i64> {
    impled!()
}
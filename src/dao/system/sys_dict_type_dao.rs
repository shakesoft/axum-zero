use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_dict_type_model::DictType;
use crate::vo::system::sys_dict_type_vo::QueryDictTypeListReq;

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


/*
 *字典类型基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(DictType {}, "sys_dict_type");


/*
 *根据id查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictType{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_dict_type");

/*
 *根据dict_type查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(DictType{select_by_dict_type(dict_type:&str) -> Option => "`where dict_type = #{dict_type} limit 1`"}, "sys_dict_type");

/*
 *分页查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictType{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dict_type");

/*
 *根据条件分页查询字典类型
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(DictType{select_dict_type_list(req:&QueryDictTypeListReq) =>"
    where 1=1
     if req.dictName != null && req.dictName != '':
      ` and dict_name like concat('%', #{req.dictName}, '%') `
     if req.dictType != null && req.dictType != '':
      ` and dict_type like concat('%', #{req.dictType}, '%') `
     if req.status != 2:
      ` and status = #{req.status} `
     if !sql.contains('count'):
      ` order by create_time desc"
},"sys_dict_type");

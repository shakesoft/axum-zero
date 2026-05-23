use log::info;
use rbatis::{Error, RBatis};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbs::value;
use crate::model::system::sys_dept_model::Dept;
use crate::vo::system::sys_dept_vo::QueryDeptListReq;

pub struct SysDeptDao;

impl SysDeptDao {

    #[function_name::named]
    pub async fn update_dept_status(rb: &RBatis, ancestors: &str, status: i8) -> Result<ExecResult, Error> {
        info!("{function_name}:{ancestors:?}:{status}",function_name = function_name!());

        // 过滤空字符串
        let ids: Vec<i64> = ancestors
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.i64())
            .collect();

        if ids.is_empty() {
            return Err(Error::from("No valid ID found in ancestors"));
        }

        let placeholders = vec!["?"; ids.len()].join(",");

        let update_sql = format!(
            "update sys_dept set status = ? , update_time = ? where id in ({})",
            placeholders
        );

        let mut params = vec![value!(status), value!(DateTime::now())];
        params.extend(ids.iter().map(|&id| value!(id)));

        let result = rb.exec(&update_sql, params).await?;
        Ok(result)
    }
}


/*
 *部门基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(Dept {}, "sys_dept");

/*
 *根据id查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Dept{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_dept");

/*
 *根据部门名称查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Dept{select_by_dept_name(dept_name:&str, parent_id:&i64) -> Option => "`where dept_name = #{dept_name} and parent_id = #{parent_id} limit 1`"}, "sys_dept");

/*
 *分页查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Dept{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_dept");

/*
 *根据条件分页查询部门
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(Dept{select_page_dept_list(req:&QueryDeptListReq) =>"
    where 1=1
     if req.deptName != null && req.deptName != '':
      ` and dept_name = #{req.deptName} `
     if req.status != 2:
      ` and status = #{req.status} `
     if !sql.contains('count'):
      ` order by sort"
},"sys_dept");

/*
 *根据部门id查询是否有下级部门
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(*) from sys_dept where status = 1 and del_flag = '1' and find_in_set(?, ancestors)")]
pub async fn select_normal_children_dept_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *根据父部门id查询下级部门数量
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(1) from sys_dept where del_flag = '1' and parent_id = ?")]
pub async fn select_dept_count(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 *查询部门是否存在用户
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
#[sql("select count(1) from sys_user where dept_id = ? and del_flag = '1'")]
pub async fn check_dept_exist_user(rb: &RBatis, id: &i64) -> rbatis::Result<i64> {
    impled!()
}

/*
 * 描述：根据部门id查询是所有下级部门
 * author：刘飞华
 * date：2025/1/6 11:29
 */
#[sql("select * from sys_dept where find_in_set(?, ancestors)")]
pub async fn select_children_dept_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Vec<Dept>> {
    impled!()
}

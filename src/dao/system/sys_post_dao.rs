use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_post_model::Post;
use crate::vo::system::sys_post_vo::QueryPostListReq;

pub struct SysPostDao;

impl SysPostDao {
    // 封装更新岗位信息状态的数据库操作
    pub async fn update_status(rb: &RBatis, ids: &Vec<i64>, status: i8) -> rbatis::Result<()> {
        let update_sql = format!("update sys_post set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }
}


/*
 *岗位信息基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(Post {}, "sys_post");


/*
 *根据id查询岗位信息
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(Post{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_post");

/*
 *根据post_code查询岗位信息
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(Post{select_by_code(post_code:&str) -> Option => "`where post_code = #{post_code} limit 1`"}, "sys_post");

/*
 *根据post_name查询岗位信息
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(Post{select_by_name(post_name:&str) -> Option => "`where post_name = #{post_name} limit 1`"}, "sys_post");

/*
 *分页查询岗位信息
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Post{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_post");

/*
 *根据条件分页查询岗位信息
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Post{select_post_list(req:&QueryPostListReq) =>"
    where 1=1
     if req.postCode != null && req.postCode != '':
      ` and post_code like concat('%', #{req.postCode}, '%') `
     if req.postName != null &&req. postName != '':
      ` and post_name like concat('%', #{req.postName}, '%') `
     if req.status != 2:
      ` and status = #{req.status} `
     if !sql.contains('count'):
      ` order by create_time desc"
},"sys_post");

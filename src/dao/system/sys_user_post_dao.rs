
/*
 *用户与岗位关联基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
use rbatis::RBatis;
use crate::model::system::sys_user_post_model::UserPost;

rbatis::crud!(UserPost {}, "sys_user_post");

/*
 *根据id查询用户与岗位关联
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(UserPost{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_user_post");

/*
 *分页查询用户与岗位关联
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(UserPost{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_user_post");

/*
 *根据条件分页查询用户与岗位关联
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(UserPost{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"sys_user_post");

/*
 *通过岗位id查询岗位使用数量
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[sql("select count(1) from sys_user_post where post_id = ? ")]
pub async fn count_user_post_by_id(rb: &RBatis, post_id: i64) -> rbatis::Result<i64> {
    impled!()
}
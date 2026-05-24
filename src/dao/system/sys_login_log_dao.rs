use rbatis::RBatis;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::vo::system::sys_login_log_vo::QueryLoginLogListReq;

struct SysLoginLogDao;

impl SysLoginLogDao {
    
}


/*
 *系统访问记录基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(LoginLog {}, "sys_login_log");

/*
 *根据id查询系统访问记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(LoginLog{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_login_log");

/*
 *分页查询系统访问记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(LoginLog{select_page() =>"
     if !sql.contains('count'):
       order by login_time desc"
},"sys_login_log");

/*
 *根据条件分页查询系统访问记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(LoginLog{select_login_log_list(req:&QueryLoginLogListReq) =>"
    where 1=1
     if req.loginName != '' && req.loginName != null:
       ` and login_name like concat('%', #{req.loginName}, '%') `
     if req.ipaddr != '' && req.ipaddr != null:
       ` and ipaddr like concat('%', #{req.ipaddr}, '%') `
     if req.browser != '' && req.browser != null:
       ` and browser like concat('%', #{req.browser}, '%') `
     if req.os != '' && req.os != null:
       ` and os like concat('%', #{req.os}, '%') `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
       ` order by login_time desc `"
},"sys_login_log");

/*
 *清空系统登录日志
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[sql("truncate table sys_login_log")]
pub async fn clean_login_log(rb: &RBatis) -> Option<i64> {
    impled!()
}

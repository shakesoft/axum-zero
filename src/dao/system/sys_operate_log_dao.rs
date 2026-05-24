use rbatis::RBatis;
use crate::model::system::sys_operate_log_model::OperateLog;
use crate::vo::system::sys_operate_log_vo::QueryOperateLogListReq;

struct SysOperateLogDao;

impl SysOperateLogDao {
    
}


/*
 *操作日志记录基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(OperateLog {}, "sys_operate_log");

/*
 *根据id查询操作日志记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(OperateLog{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_operate_log");

/*
 *分页查询操作日志记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(OperateLog{select_page() =>"
     if !sql.contains('count'):
       order by operate_time desc"
},"sys_operate_log");

/*
 *根据条件分页查询操作日志记录
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(OperateLog{select_page_by_name(
    req:&QueryOperateLogListReq) =>"
    where 1=1
     if req.title != '' && req.title != null:
       ` and title like concat('%', #{req.title}, '%') `
     if req.businessType != 4:
       ` and business_type = #{req.businessType} `
     if req.method != '' && req.method != null:
       ` and method = #{req.method} `
     if req.requestMethod != '' && req.requestMethod != null:
       ` and request_method = #{req.requestMethod} `
     if req.operatorType != 3:
       ` and operator_type = #{req.operatorType} `
     if req.operateName != '' && req.operateName != null:
       ` and operate_name = #{req.operateName} `
     if req.deptName != '' && req.deptName != null:
       ` and dept_name = #{req.deptName} `
     if req.operateUrl != '' && req.operateUrl != null:
       ` and operate_url = #{req.operateUrl} `
     if req.operateIp != '' && req.operateIp != null:
       ` and operate_ip = #{req.operateIp} `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
       ` order by operate_time desc `"
},"sys_operate_log");

/*
 *清空操作日志
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[sql("truncate table sys_operate_log")]
pub async fn clean_operate_log(rb: &RBatis) -> Option<i64> {
    impled!()
}

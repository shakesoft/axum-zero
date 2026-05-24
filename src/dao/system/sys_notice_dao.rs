use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::QueryNoticeListReq;

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


/*
 *通知公告表基本操作
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
rbatis::crud!(Notice {}, "sys_notice");

/*
 *根据id查询通知公告表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(Notice{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_notice");

/*
 *根据公告标题查询通知公告表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select!(Notice{select_by_title(title:&str) -> Option => "`where notice_title = #{title} limit 1`"}, "sys_notice");

/*
 *根据条件分页查询通知公告表
 *author：罗京生
 *date：2024/12/25 10:01:11
 */
impl_select_page!(Notice{select_sys_notice_list(req:&QueryNoticeListReq) =>"
    where 1=1
     if req.noticeTitle != '' && req.noticeTitle != null:
       ` and notice_title like concat('%', #{req.noticeTitle}, '%') `
     if req.noticeType != 0:
      ` and notice_type = #{req.noticeType} `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
       ` order by create_time desc `"
},"sys_notice");

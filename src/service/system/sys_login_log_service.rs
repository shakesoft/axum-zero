use crate::common::error::{AppError, AppResult, ServiceResult, ServiceResultPage};
use crate::common::result::{ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_login_log_dao;
use crate::model::system::sys_login_log_model::LoginLog;
use crate::utils::user_agent_util::UserAgentUtil;
use crate::vo::system::sys_login_log_vo::{DeleteLoginLogReq, LoginLogResp, QueryLoginLogDetailReq, QueryLoginLogListReq};
use axum::Json;
use log::info;
use rbatis::plugin::page::PageRequest;
use rbatis::RBatis;
use rbs::value;

pub struct SysLoginLogService;

impl SysLoginLogService {
    pub async fn delete_sys_login_log(rb: &RBatis, item: DeleteLoginLogReq) -> ServiceResult<String> {
        LoginLog::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result_empty())?
    }

    pub async fn clean_sys_login_log(rb: &RBatis) -> ServiceResult<String> {
        sys_login_log_dao::clean_login_log(rb).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_login_log_detail(rb: &RBatis, item: QueryLoginLogDetailReq) -> ServiceResult<LoginLogResp> {
        LoginLog::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("系统访问记录不存在")),
            |x| {
                let data: LoginLogResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_login_log_list(rb: &RBatis, item: QueryLoginLogListReq) -> ServiceResultPage<LoginLogResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        LoginLog::select_login_log_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<LoginLogResp>>(), x.total))?
    }

    /*
     *添加登录日志
     *author：罗京生
     *date：2025/01/02 17:01:13
     */
    pub async fn add_login_log(rb: &RBatis, name: String, status: i8, msg: &str, agent: UserAgentUtil) {
        let sys_login_log = LoginLog {
            id: None,                             //访问ID
            login_name: name,                     //登录账号
            ipaddr: "todo".to_string(),           //登录IP地址
            login_location: "todo".to_string(),   //登录地点
            platform: agent.platform,             //平台信息
            browser: agent.browser,               //浏览器类型
            version: agent.version,               //浏览器版本
            os: agent.os,                         //操作系统
            arch: agent.arch,                     //体系结构信息
            engine: agent.engine,                 //渲染引擎信息
            engine_details: agent.engine_details, //渲染引擎详细信息
            extra: agent.extra,                   //其他信息（可选）
            status,                               //登录状态(0:失败,1:成功)
            msg: msg.to_string(),                 //提示消息
            login_time: None,                     //访问时间
        };

        match LoginLog::insert(rb, &sys_login_log).await {
            Ok(_u) => info!("add_login_log success: {:?}", sys_login_log),
            Err(err) => log::error!("add_login_log error params: {:?}, error message: {:?}", sys_login_log, err),
        }
    }
}

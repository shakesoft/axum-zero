// author：罗京生
// createTime：2024/12/12 14:41:44

use crate::vo::system::sys_user_vo::{QueryUserListReq, UserReq, UserResp};
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::Error;
use serde::{Deserialize, Serialize};
/*
 *用户信息
 *author：罗京生
 *date：2024/12/12 14:41:44
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,                   //主键
    pub mobile: String,                    //手机
    pub user_name: String,                 //用户账号
    pub nick_name: String,                 //用户昵称
    pub user_type: Option<String>,         //用户类型（00系统用户）
    pub email: String,                     //用户邮箱
    pub avatar: Option<String>,            //头像路径
    pub password: String,                  //密码
    pub status: i8,                        //状态(1:正常，0:禁用)
    pub dept_id: i64,                      //部门ID
    pub login_ip: String,                  //最后登录IP
    pub login_date: Option<DateTime>,      //最后登录时间
    pub login_browser: String,             //浏览器类型
    pub login_os: String,                  //操作系统
    pub pwd_update_date: Option<DateTime>, //密码最后更新时间
    pub remark: Option<String>,            //备注
    pub del_flag: i8,                      //删除标志（0代表删除 1代表存在）
    pub create_time: Option<DateTime>,     //创建时间
    pub update_time: Option<DateTime>,     //修改时间
}


impl From<UserReq> for User {
    fn from(item: UserReq) -> Self {
        let mut model = User {
            id: None,                                    //主键
            mobile: item.mobile,                         //手机
            user_name: item.user_name,                   //用户账号
            nick_name: item.nick_name,                   //用户昵称
            user_type: Some("01".to_string()),           //用户类型（00系统用户）
            email: item.email,                           //用户邮箱
            avatar: item.avatar,                         //头像路径
            password: item.password.unwrap_or_default(), //密码
            status: item.status,                         //状态(1:正常，0:禁用)
            dept_id: item.dept_id,                       //部门ID
            login_ip: "".to_string(),                    //最后登录IP
            login_date: None,                            //最后登录时间
            login_browser: "".to_string(),               //浏览器类型
            login_os: "".to_string(),                    //操作系统
            pwd_update_date: None,                       //密码最后更新时间
            remark: item.remark,                         //备注
            del_flag: 1,                                 //删除标志（0代表删除 1代表存在）
            create_time: None,                           //创建时间
            update_time: None,                           //修改时间
        };
        if let None = item.id {
            model.create_time = Some(DateTime::now());
        } else {
            model.update_time = Some(DateTime::now());
        }
        model
    }
}

impl Into<UserResp> for User {
    fn into(self) -> UserResp {
        UserResp {
            id: self.id,                                   //主键
            mobile: self.mobile,                           //手机
            user_name: self.user_name,                     //姓名
            nick_name: self.nick_name,                     //用户昵称
            user_type: self.user_type.unwrap_or_default(), //用户类型（00系统用户）
            email: self.email,                             //用户邮箱
            avatar: self.avatar,                           //头像路径
            status: self.status,                           //状态(1:正常，0:禁用)
            dept_id: self.dept_id,                         //部门ID
            login_ip: self.login_ip,                       //最后登录IP
            login_date: self.login_date,                   //最后登录时间
            login_browser: self.login_browser,             //浏览器类型
            login_os: self.login_os,                       //操作系统
            pwd_update_date: self.pwd_update_date,         //密码最后更新时间
            remark: self.remark,                           //备注
            create_time: self.create_time,                 //创建时间
            update_time: self.update_time,                 //修改时间
            dept_info: None,
            post_ids: None,
        }
    }
}


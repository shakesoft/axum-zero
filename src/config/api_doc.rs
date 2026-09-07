use utoipa::OpenApi;
use crate::{handler, vo};

// ponytail: 由 src/route 的路由表生成，新增路由后同步补一行；等到手工维护出错才值得写 build.rs
#[derive(OpenApi)]
#[openapi(
    paths(
        handler::system::sys_dept_handler::add_sys_dept,
        handler::system::sys_dept_handler::delete_sys_dept,
        handler::system::sys_dept_handler::delete_sys_dept1,
        handler::system::sys_dept_handler::query_sys_dept_detail,
        handler::system::sys_dept_handler::query_sys_dept_list,
        handler::system::sys_dept_handler::update_sys_dept,
        handler::system::sys_dept_handler::update_sys_dept_status,
        handler::system::sys_dict_data_handler::add_sys_dict_data,
        handler::system::sys_dict_data_handler::delete_sys_dict_data,
        handler::system::sys_dict_data_handler::query_sys_dict_data_detail,
        handler::system::sys_dict_data_handler::query_sys_dict_data_list,
        handler::system::sys_dict_data_handler::update_sys_dict_data,
        handler::system::sys_dict_data_handler::update_sys_dict_data_status,
        handler::system::sys_dict_type_handler::add_sys_dict_type,
        handler::system::sys_dict_type_handler::delete_sys_dict_type,
        handler::system::sys_dict_type_handler::query_sys_dict_type_detail,
        handler::system::sys_dict_type_handler::query_sys_dict_type_list,
        handler::system::sys_dict_type_handler::update_sys_dict_type,
        handler::system::sys_dict_type_handler::update_sys_dict_type_status,
        handler::system::sys_login_log_handler::clean_sys_login_log,
        handler::system::sys_login_log_handler::delete_sys_login_log,
        handler::system::sys_login_log_handler::query_sys_login_log_detail,
        handler::system::sys_login_log_handler::query_sys_login_log_list,
        handler::system::sys_menu_handler::add_sys_menu,
        handler::system::sys_menu_handler::delete_sys_menu,
        handler::system::sys_menu_handler::query_sys_menu_detail,
        handler::system::sys_menu_handler::query_sys_menu_list,
        handler::system::sys_menu_handler::query_sys_menu_list_simple,
        handler::system::sys_menu_handler::update_sys_menu,
        handler::system::sys_menu_handler::update_sys_menu_status,
        handler::system::sys_notice_handler::add_sys_notice,
        handler::system::sys_notice_handler::delete_sys_notice,
        handler::system::sys_notice_handler::query_sys_notice_detail,
        handler::system::sys_notice_handler::query_sys_notice_list,
        handler::system::sys_notice_handler::query_sys_notice_request,
        handler::system::sys_notice_handler::update_sys_notice,
        handler::system::sys_notice_handler::update_sys_notice_status,
        handler::system::sys_operate_log_handler::clean_sys_operate_log,
        handler::system::sys_operate_log_handler::delete_sys_operate_log,
        handler::system::sys_operate_log_handler::query_sys_operate_log_detail,
        handler::system::sys_operate_log_handler::query_sys_operate_log_list,
        handler::system::sys_post_handler::add_sys_post,
        handler::system::sys_post_handler::delete_sys_post,
        handler::system::sys_post_handler::query_sys_post_detail,
        handler::system::sys_post_handler::query_sys_post_list,
        handler::system::sys_post_handler::update_sys_post,
        handler::system::sys_post_handler::update_sys_post_status,
        handler::system::sys_role_handler::add_sys_role,
        handler::system::sys_role_handler::batch_auth_user,
        handler::system::sys_role_handler::batch_cancel_auth_user,
        handler::system::sys_role_handler::cancel_auth_user,
        handler::system::sys_role_handler::delete_sys_role,
        handler::system::sys_role_handler::query_allocated_list,
        handler::system::sys_role_handler::query_role_menu,
        handler::system::sys_role_handler::query_sys_role_detail,
        handler::system::sys_role_handler::query_sys_role_list,
        handler::system::sys_role_handler::query_unallocated_list,
        handler::system::sys_role_handler::update_role_menu,
        handler::system::sys_role_handler::update_sys_role,
        handler::system::sys_role_handler::update_sys_role_status,
        handler::system::sys_user_handler::add_sys_user,
        handler::system::sys_user_handler::delete_sys_user,
        handler::system::sys_user_handler::login,
        handler::system::sys_user_handler::query_sys_user_detail,
        handler::system::sys_user_handler::query_sys_user_list,
        handler::system::sys_user_handler::query_user_menu,
        handler::system::sys_user_handler::query_user_role,
        handler::system::sys_user_handler::reset_sys_user_password,
        handler::system::sys_user_handler::update_sys_user,
        handler::system::sys_user_handler::update_sys_user_password,
        handler::system::sys_user_handler::update_sys_user_status,
        handler::system::sys_user_handler::update_user_role,
    ),
    components(
        // BaseResponse<()> 的 data 指向 TupleUnit, 自动收集拿不到, 其余 schema 由 paths 自动收集
        schemas(utoipa::TupleUnit)
    ),
    tags(
        (name = "axum-zero", description = "OpenAPI")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn openapi_refs_resolve() {
        let json = ApiDoc::openapi().to_json().unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        let comps: Vec<String> = v["components"]["schemas"].as_object().unwrap().keys().cloned().collect();
        let mut missing = vec![];
        for r in regex::Regex::new(r###""\$ref": *"#/components/schemas/([^"]+)""###).unwrap().captures_iter(&json) {
            let n = r[1].to_string();
            if !comps.contains(&n) { missing.push(n); }
        }
        missing.sort(); missing.dedup();
        assert!(comps.len() > 30, "schemas auto-collection broke: {}", comps.len());
        assert!(missing.is_empty(), "dangling refs: {missing:?}");
    }
}

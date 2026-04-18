use log::info;
use rbatis::{Error, RBatis};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::rbdc::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbs::value;

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
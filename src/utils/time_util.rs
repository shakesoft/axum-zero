use rbatis::rbdc::DateTime;

/*
 *时间转字符串
 *author：刘飞华
 *date：2025/01/02 14:38:11
 */
pub fn time_to_string(t: Option<DateTime>) -> String {
    match t {
        None => "".to_string(),
        Some(x) => x.format("YYYY-MM-DD hh:mm:ss"),
    }
}

/// 安全地将 Unix 时间戳（秒）转换为 UTC DateTime
pub fn timestamp_to_utc(timestamp: i64) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::from_timestamp(timestamp, 0)
}

/// 安全地将 Unix 时间戳（秒）转换为本地 DateTime
pub fn timestamp_to_local(timestamp: i64) -> Option<chrono::DateTime<chrono::Local>> {
    chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|utc| utc.with_timezone(&chrono::Local))
}

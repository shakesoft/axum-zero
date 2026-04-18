#[macro_use]
extern crate rbatis;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;
pub mod dao;
pub mod workflow;
pub mod service;
mod aop;

use std::net::SocketAddr;
use axum::{middleware as md, Json, Router};
use crate::route::system::sys_dept_route::build_sys_dept_route;
use crate::route::system::sys_dict_data_route::build_sys_dict_data_route;
use crate::route::system::sys_dict_type_route::build_sys_dict_type_route;
use crate::route::system::sys_login_log_route::build_sys_login_log_route;
use crate::route::system::sys_notice_route::build_sys_notice_route;
use crate::route::system::sys_operate_log_route::build_sys_operate_log_route;
use crate::route::system::sys_post_route::build_sys_post_route;
use crate::utils::redis_util::init_redis;
use config::{Config, File};
use middleware::auth::auth;
use rbatis::RBatis;
use redis::Client;
use route::system::sys_menu_route::build_sys_menu_route;
use route::system::sys_role_route::build_sys_role_route;
use route::system::sys_user_route::build_sys_user_route;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use axum::body::Body;
use axum::http::{Method, Request, Response};
use axum::response::IntoResponse;
// use tower::{ServiceBuilder};
use tower_http::{
    catch_panic::CatchPanicLayer, classify::ServerErrorsFailureClass, trace::TraceLayer,
};
use tracing::{error, info, Span};
use tracing_subscriber;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;
use utils::db::init_db;
use crate::common::error::AppError;
// use crate::middleware::error::{ handle_middleware_error};
// use crate::middleware::swagger::swagger_auth;
use axum::routing::get;
use chrono::Utc;
use reqwest::StatusCode;
// use garde::rules::ip::IpKind::Any;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tracing_appender::rolling;
use crate::common::result::ok_result_msg;
use crate::workflow::state::traffic_light::TrafficLight;
// use crate::common::daily_logfile::DailyLogFile;
// use crate::handler::system::sys_user_handler::reset_sys_user_password;

// 定义应用状态结构体，包含数据库连接池
#[derive(Debug)]
pub struct AppState {
    pub batis: RBatis,
    pub redis: Client,
}

impl AppState {
    /// 返回一些安全的诊断信息（不依赖内部类型的 Debug 实现）
    pub fn diag(&self) -> String {
        let b_ptr = &self.batis as *const _ as *const ();
        let r_ptr = &self.redis as *const _ as *const ();
        format!("AppState diag: batis_ptr={:p}, redis_ptr={:p}", b_ptr, r_ptr)
    }
}

// 配置结构体，包含服务器和数据库配置
#[derive(Debug, Deserialize)]
struct Config1 {
    server: ServerConfig,
    db: DbConfig,
    redis: RedisConfig,
}

// 服务器配置结构体，包含服务器地址
#[derive(Debug, Deserialize)]
struct ServerConfig {
    addr: String,
}

// 数据库配置结构体，包含数据库URL
#[derive(Debug, Deserialize)]
struct DbConfig {
    url: String,
}

#[derive(Debug, Deserialize)]
struct RedisConfig {
    url: String,
}

//这个宏用于生成OpenAPI文档，有没有办法可以通过脚本自动生成？
#[derive(OpenApi)]
#[openapi(
    paths(
        handler::system::sys_dept_handler::add_sys_dept,
        handler::system::sys_dept_handler::delete_sys_dept,
        handler::system::sys_dept_handler::update_sys_dept,
        handler::system::sys_dept_handler::update_sys_dept_status,
        handler::system::sys_dept_handler::query_sys_dept_detail,
        handler::system::sys_dept_handler::query_sys_dept_list,
    ),
    components(
        schemas(
            vo::system::sys_dept_vo::DeptReq,
            vo::system::sys_dept_vo::DeleteDeptReq
        )
    ),
    tags(
        (name = "axum-admin", description = "OpenAPI")
    )
)]
struct ApiDoc;

// 主函数，使用tokio异步运行时
#[tokio::main]
async fn main() {
    // #[cfg(debug_assertions)]
    // #[cfg(not(debug_assertions))]
    // {
    //     // 初始化日志配置
    //     log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    // }
    let light = TrafficLight::new(());
    // Type is TrafficLight<Red>


    let light = light.next().unwrap();
    // Type is TrafficLight<Green>

    let light = light.next().unwrap();
    // Type is TrafficLight<Yellow>


    let light = TrafficLight::new(());
    let mut dynamic_light = light.into_dynamic();
    // let dd = dynamic_light.into_yellow().unwrap();
    // let ee = dd.into_dynamic().current_state();
    // let cc = dynamic_light.into_green().unwrap();
    let bb = dynamic_light.current_state();
    println!("{}",bb);

    // #[derive(Debug)]
    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }
    // let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // let i = ImportantExcerpt {
    //     part: first_sentence,
    // };
    // println!("{}",i.part);

    let file_appender = rolling::daily("log", "axum-admin.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // let daily_writer = DailyLogFile::new("log", "axum-admin", "log");
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // .with_writer(daily_writer)
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // 加载和解析配置文件
    let config = Config::builder().add_source(File::with_name("config.toml")).build().unwrap().try_deserialize::<Config1>().unwrap();
    println!("Config: {:?}", config);

    // 初始化数据库连接
    let rb = init_db(config.db.url.as_str()).await;
    let rd = init_redis(config.redis.url.as_str()).await;

    // 创建共享应用状态，包含数据库连接池
    let shared_state = Arc::new(AppState { batis: rb, redis: rd });

    // 跨域中间件
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
		
		/*
		let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT]);
		**/

    // Swagger-UI
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());

    // 请求超时中间件
    let timeout = TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT,Duration::from_secs(6));

    // 请求追踪中间件
    let trace = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| {
                let request_id = Uuid::new_v4().to_string();
                tracing::info_span!("http-request: ", %request_id)
            }
        )
        .on_request(|request: &Request<Body>, _span: &Span| {
                info!("request: {} {}", request.method(), request.uri().path())
            }
        )
        .on_response(|response: &Response<Body>, latency: Duration, _span: &Span| {
                info!("response: {} {:?}", response.status(), latency);
            },
        )
        .on_failure(|err: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                error!("Don't panic, C'est la vie. {}", err)
            },
        );

    // 全局异常捕获中间件
    let panic = CatchPanicLayer::custom(|panic_info: Box<dyn std::any::Any + Send>| {
        error!("Custom panic hook: {panic_info:?}");// 这里可以上报日志、监控或做其他操作
        AppError::default().into_response() // 保持原有的响应行为
    });

    // 首页路由
    let home_router = Router::new().route("/", get(async||-> &'static str {
        "Hello axum-admin!"
    }));

    let index_router = Router::new().route("/index", get(async||-> String{
        let json =json_data();
        return json.to_string();
    }));


    let test_router = Router::new().route("/test", get(async||-> String {
        let body = reqwest::get("http://dev.muche365.com/enter/lutong/order-request")
            .await.unwrap()
            .text()
            .await.unwrap();

        //let mut body = String::new();
        // res.read_to_string(&mut body)?;
        info!("{body}");
        println!("{}", body);
        body
    }));

    let test_router1 = Router::new().route("/test1", get(async || {
        ok_result_msg("成功啦")
    }));

    // 构建应用路由，并合并多个子路由
    let app = Router::new().merge(swagger_ui).merge(home_router).merge(index_router).merge(test_router).merge(test_router1)//.route_layer(md::from_fn(swagger_auth))
        .nest(
        "/api",
        Router::new()
            .merge(build_sys_user_route())
            .merge(build_sys_role_route())
            .merge(build_sys_menu_route())
            .merge(build_sys_dept_route())
            .merge(build_sys_dict_type_route())
            .merge(build_sys_dict_data_route())
            .merge(build_sys_post_route())
            .merge(build_sys_login_log_route())
            .merge(build_sys_operate_log_route())
            .merge(build_sys_notice_route())
            .layer(cors)
            .layer(timeout) // 设置请求超时时间为3秒
            .layer(trace)
            .layer(panic)
            .route_layer(md::from_fn_with_state(Arc::clone(&shared_state), auth)) // 添加认证中间件
            .with_state(shared_state), // 设置共享状态
    );

    // 以下代码适用于axum 0.6.x版本
    // 定义服务器监听地址
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // 使用绑定地址启动服务器
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // 以下代码适用于axum 0.7.x版本
    // 创建TCP监听器
    let listener = tokio::net::TcpListener::bind(config.server.addr).await.unwrap();
    // 使用监听器启动服务器
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

pub fn json_data()->&'static str{
    let data:&'static str = r#"{
  "accidentType": "0",
  "carModel": "WEYWEY全新蓝山",
  "caseCity": "上海市",
  "caseCounty": "松江区",
  "caseDestCity": "上海市",
  "caseDestCounty": "嘉定区",
  "caseDestLat": "31.303306",
  "caseDestLng": "121.32953",
  "caseDestLocation": "上海市嘉定区南翔地铁站",
  "caseDestState": "上海",
  "caseLat": "31.024269",
  "caseLng": "121.23782",
  "caseLocation": "上海市松江区松江体育中心地铁站华中公寓9号楼",
  "caseState": "上海",
  "chargeType": "01",
  "customerName": "技术部测试",
  "customerPhone": "12100000028",
  "customerPhoneForDispatcher": "12100000028",
  "customerPhoneForDriver": "12100000028",
  "dealerCode": "CC23221",
  "dealerHotline": "0315-5390919",
  "dealerName": "张家港市祥瑞联发汽车贸易有限公司",
  "dealerPhone": "03155390915",
  "freeMiles": "200",
  "orderNo": "19122601230949313472",
  "plateNo": "沪AF08234",
  "recipientName": "接车人测试",
  "recipientPhone": "03155390916",
  "reportTime": "2026-01-23 09:49:31",
  "requestType": "01",
  "rescueType": "2",
  "reservationFlg": "0",
  "serviceDemand": [
    {
      "demandName": "免路桥费",
      "demandCode": "001"
    },
    {
      "demandName": "本单在完成前，修改救援地或目的地需要凭证附件",
      "demandCode": "005"
    }
  ],
  "vinNo": "LGWFF7A55MJ066983",
  "vipLevel": "VIP"
}"#;
    data
}

pub async fn post_json(
    url: &str,
    json_body: &str,
    app_key: &str,
    timestamp: &str,
    sign: &str,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("appKey", app_key)
        .header("timestamp", timestamp)
        .header("sign", sign)
        .body(json_body.to_string())
        .send()
        .await?
        .error_for_status()?   // 自动检查 4xx/5xx
        .text()
        .await?;

    Ok(resp)
}
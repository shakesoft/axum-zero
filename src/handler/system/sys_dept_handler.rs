use crate::common::extractor::ValidatedJson;
use crate::common::result::{ok, BaseResponse, EmptyResponse};
use crate::vo::system::sys_dept_vo::*;
use crate::AppState;
use aspect_macros::aspect;
use aspect_std::{LoggingAspect, TimingAspect};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use log::info;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;
use shaku::HasComponent;
use std::ops::Deref;
use std::sync::Arc;
use tracing::instrument;
// use std::time::Duration;
// use tokio::time::sleep;
use crate::inject::autofac::{AutoFacModule, HelloWorld, IDateWriter, IOutput};
use crate::inject::inject_component::Inject;
use crate::inject::inject_provided::InjectProvided;
use crate::service::system::sys_dept_service::SysDeptService;
use crate::vo::system::sys_user_vo::UserSession;
/*
 *添加部门表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/addDept",
    request_body = DeptReq,
    responses((status = 200, description = "successfully", body = EmptyResponse))
)]
// #[instrument]
// #[function_name::named]
pub async fn add_sys_dept(State(state): State<Arc<AppState>>, ValidatedJson(item): ValidatedJson<DeptReq>) -> impl IntoResponse {
    // panic!("test");
    // sleep(Duration::from_secs(8)).await;
    // return AppError::interrupt();
    // info!("add sys_dept params: {:?}", &item);
    // info!("{function_name}:{item:?}",function_name = function_name!());
    // info!("{function_name}:{item:?}",function_name = function_name!());

    // let container = &state.container;
    // let service: &dyn IDateWriter = container.resolve_ref();
    // service.write_date();
    // service.get_date();
    //
    // let a = SysDeptService::test_closure(10,2);
    // let b  = a.0();
    // let c  = a.1();
    // let d  = a.2();
    // let e  = a.3();

    // info!("{}",b);
    // info!("{}",c);
    // info!("{}",d);
    // info!("{}",e);
    // info!("{}: {}: {}: {}", b, c, d, e);
    // info!("{}: {:?}", function_name!(), item);
    // info!("{}: {item:?}",function_name!());
    let rb = &state.batis;

    SysDeptService::add_sys_dept(rb, item).await
}

/*
 *删除部门表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/deleteDept",
    request_body = DeleteDeptReq,
    responses((status = 200, description = "successfully", body = EmptyResponse))
)]
#[function_name::named]
pub async fn delete_sys_dept(State(state): State<Arc<AppState>>, Extension(_session): Extension<UserSession>, Json(item): Json<DeleteDeptReq>) -> impl IntoResponse {
    // info!("{function_name}:{item:?}",function_name = function_name!());
    info!("{}: {:?}", function_name!(), item);
    // panic!("test");
    let rb = &state.batis;

    SysDeptService::delete_sys_dept(rb, item).await
}

#[utoipa::path(
    post,
    path = "/api/system/dept/deleteDept1",
    request_body = DeleteDeptReq,
    responses((status = 200, description = "successfully", body = EmptyResponse))
)]
// #[function_name::named]
pub async fn delete_sys_dept1(
    writer: Inject<AutoFacModule, dyn IDateWriter>,
    hello_world: InjectProvided<AutoFacModule, dyn HelloWorld>,
    State(state): State<Arc<AppState>>,
    Extension(session): Extension<UserSession>,
    Json(item): Json<DeleteDeptReq>,
) -> impl IntoResponse {
    writer.write_date();
    writer.get_date();
    let result = hello_world.greet();
    info!("{}", result);

    // info!("{function_name}:{item:?}",function_name = function_name!());
    // info!("{}: {:?}", function_name!(), item);
    // let user_id = &session.user_id;
    ok()
    // let permissons = &session.permissions;
    // let rb = &state.batis;
    //
    // if select_dept_count(rb, &item.id).await? > 0 {
    //     return Err(AppError::BusinessError("存在下级部门,不允许删除"));
    // }
    //
    // if check_dept_exist_user(rb, &item.id).await? > 0 {
    //     return Err(AppError::BusinessError("部门存在用户,不允许删除"));
    // }
    //
    // Dept::delete_by_map(rb, value! {"id": &item.id}).await.map(|_| ok())?
}

/*
 *更新部门表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/updateDept",
    request_body = DeptReq,
    responses((status = 200, description = "successfully", body = EmptyResponse))
)]
#[function_name::named]
pub async fn update_sys_dept(State(state): State<Arc<AppState>>, ValidatedJson(item): ValidatedJson<DeptReq>) -> impl IntoResponse {
    info!("{}: {:?}", function_name!(), item);
    let rb = &state.batis;

    SysDeptService::update_sys_dept(rb, item).await
}

/*
 *更新部门表状态
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/updateDeptStatus",
    request_body = UpdateDeptStatusReq,
    responses((status = 200, description = "successfully", body = EmptyResponse))
)]
#[function_name::named]
pub async fn update_sys_dept_status(State(state): State<Arc<AppState>>, Json(item): Json<UpdateDeptStatusReq>) -> impl IntoResponse {
    // info!("{function_name}:{item:?}",function_name = function_name!());
    info!("{}: {:?}", function_name!(), item);
    let rb = &state.batis;

    SysDeptService::update_sys_dept_status(rb, item).await
}
/*
 *查询部门表详情
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/queryDeptDetail",
    request_body = QueryDeptDetailReq,
    responses((status = 200, description = "successfully", body = BaseResponse<DeptResp>))
)]
#[function_name::named]
pub async fn query_sys_dept_detail(State(state): State<Arc<AppState>>, Json(item): Json<QueryDeptDetailReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDeptService::query_sys_dept_detail(rb, item).await
}

/*
 *查询部门表列表
 *author：罗京生
 *date：2024/12/25 11:36:48
 */
#[utoipa::path(
    post,
    path = "/api/system/dept/queryDeptList",
    request_body = QueryDeptListReq,
    responses((status = 200, description = "successfully", body = BaseResponse<Vec<DeptResp>>))
)]
#[function_name::named]
// #[aspect(TimingAspect::new())]
pub async fn query_sys_dept_list(State(state): State<Arc<AppState>>, Json(item): Json<QueryDeptListReq>) -> impl IntoResponse {
    info!("{function_name}:{item:?}", function_name = function_name!());
    let rb = &state.batis;

    SysDeptService::query_sys_dept_list(rb, item).await
}

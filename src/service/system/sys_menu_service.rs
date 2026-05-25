use crate::common::error::{AppError, ServiceResult};
use crate::common::result::{ok_result_empty, ok_result_data};
use crate::dao::system::sys_menu_dao;
use crate::dao::system::sys_menu_dao::SysMenuDao;
use crate::dao::system::sys_role_menu_dao;
use crate::model::system::sys_menu_model::Menu;
use crate::vo::system::sys_menu_vo::{DeleteMenuReq, MenuListSimpleDataResp, MenuReq, MenuResp, QueryMenuDetailReq, QueryMenuListReq, UpdateMenuStatusReq};
use rbatis::RBatis;
use rbs::value;

pub struct SysMenuService;

impl SysMenuService {
    pub async fn add_sys_menu(rb: &RBatis, mut item: MenuReq) -> ServiceResult<String> {
        if Menu::select_by_menu_name(rb, &item.menu_name).await?.is_some() {
            return Err(AppError::BusinessError("菜单名称已存在"));
        }

        if let Some(url) = item.menu_url.clone() {
            if Menu::select_by_menu_url(rb, &url).await?.is_some() {
                return Err(AppError::BusinessError("路由路径已存在"));
            }
        }

        item.id = None;
        Menu::insert(rb, &Menu::from(item)).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_menu(rb: &RBatis, item: DeleteMenuReq) -> ServiceResult<String> {
        if sys_menu_dao::select_count_menu_by_parent_id(rb, &item.id).await? > 0 {
            return Err(AppError::BusinessError("存在子菜单,不允许删除"));
        }

        if sys_role_menu_dao::select_count_menu_by_menu_id(rb, &item.id).await? > 0 {
            return Err(AppError::BusinessError("菜单已分配,不允许删除"));
        }

        Menu::delete_by_map(rb, value! {"id": &item.id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_menu(rb: &RBatis, item: MenuReq) -> ServiceResult<String> {
        let id = item.id;

        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Menu::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("菜单信息不存在"));
        }

        if let Some(x) = Menu::select_by_menu_name(rb, &item.menu_name).await? {
            if x.id != id {
                return Err(AppError::BusinessError("菜单名称已存在"));
            }
        }

        if let Some(url) = item.menu_url.clone() {
            if let Some(x) = Menu::select_by_menu_url(rb, &url).await? {
                if x.id != id {
                    return Err(AppError::BusinessError("路由路径已存在"));
                }
            }
        }

        Menu::update_by_map(rb, &Menu::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_menu_status(rb: &RBatis, item: UpdateMenuStatusReq) -> ServiceResult<String> {
        SysMenuDao::update_menu_status(rb, item.status, &item.ids).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_menu_detail(rb: &RBatis, item: QueryMenuDetailReq) -> ServiceResult<MenuResp> {
        Menu::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("菜单信息不存在")),
            |x| {
                let data: MenuResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_menu_list(rb: &RBatis, _item: QueryMenuListReq) -> ServiceResult<Vec<MenuResp>> {
        Menu::select_all(rb).await.map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<MenuResp>>()))?
    }

    pub async fn query_sys_menu_list_simple(rb: &RBatis) -> ServiceResult<Vec<MenuListSimpleDataResp>> {
        let list = Menu::select_menu_list(rb).await?;

        let mut menu_list: Vec<MenuListSimpleDataResp> = Vec::new();

        list.into_iter()
            .map(|x| MenuListSimpleDataResp {
                id: x.id,
                menu_name: x.menu_name,
                parent_id: x.parent_id,
            })
            .for_each(|x| menu_list.push(x));

        ok_result_data(menu_list)
    }
}

use crate::common::error::{AppError, ServiceResult, ServiceResultPage};
use crate::common::result::{ok_result_empty, ok_result_data, ok_result_page};
use crate::dao::system::sys_role_dao::SysRoleDao;
use crate::dao::system::{sys_role_menu_dao, sys_user_dao, sys_user_role_dao};
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_menu_model::RoleMenu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_role_model::UserRole;
use crate::vo::system::sys_role_vo::*;
use crate::vo::system::sys_user_vo::UserResp;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbatis::RBatis;
use rbs::value;

pub struct SysRoleService;

impl SysRoleService {
    pub async fn add_sys_role(rb: &RBatis, mut item: RoleReq) -> ServiceResult<String> {
        if Role::select_by_role_name(rb, &item.role_name).await?.is_some() {
            return Err(AppError::BusinessError("角色名称已存在"));
        }

        if Role::select_by_role_key(rb, &item.role_key).await?.is_some() {
            return Err(AppError::BusinessError("角色权限已存在"));
        }

        item.id = None;
        Role::insert(rb, &Role::from(item)).await.map(|_| ok_result_empty())?
    }

    pub async fn delete_sys_role(rb: &RBatis, item: DeleteRoleReq) -> ServiceResult<String> {
        let ids = item.ids.clone();

        if ids.contains(&1) {
            return Err(AppError::BusinessError("不允许操作超级管理员角色"));
        }

        for id in ids {
            if let None = Role::select_by_id(rb, &id).await? {
                return Err(AppError::BusinessError("角色不存在,不能删除"));
            }

            if sys_user_role_dao::count_user_role_by_role_id(rb, id).await? > 0 {
                return Err(AppError::BusinessError("分配,不能删除"));
            }
        }

        SysRoleDao::delete_role_and_relations(rb, &item.ids).await?;
        ok_result_empty()
    }

    pub async fn update_sys_role(rb: &RBatis, item: RoleReq) -> ServiceResult<String> {
        let id = item.id;
        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if id == Some(1) {
            return Err(AppError::BusinessError("不允许操作超级管理员角色"));
        }

        if Role::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
            return Err(AppError::BusinessError("角色不存在"));
        }

        if let Some(x) = Role::select_by_role_name(rb, &item.role_name).await? {
            if x.id != id {
                return Err(AppError::BusinessError("角色名称已存在"));
            }
        }

        if let Some(x) = Role::select_by_role_key(rb, &item.role_key).await? {
            if x.id != id {
                return Err(AppError::BusinessError("角色权限已存在"));
            }
        }

        Role::update_by_map(rb, &Role::from(item), value! {"id": &id}).await.map(|_| ok_result_empty())?
    }

    pub async fn update_sys_role_status(rb: &RBatis, item: UpdateRoleStatusReq) -> ServiceResult<String> {
        if item.ids.contains(&1) {
            return Err(AppError::BusinessError("不允许操作超级管理员角色"));
        }

        SysRoleDao::update_status(rb, &item.ids, item.status).await.map(|_| ok_result_empty())?
    }

    pub async fn query_sys_role_detail(rb: &RBatis, item: QueryRoleDetailReq) -> ServiceResult<RoleResp> {
        Role::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("角色不存在")),
            |x| {
                let data: RoleResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_role_list(rb: &RBatis, item: QueryRoleListReq) -> ServiceResultPage<RoleResp> {
        let page = &PageRequest::new(item.page_no, item.page_size);

        Role::select_sys_role_list(rb, page, &item)
            .await
            .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<RoleResp>>(), x.total))?
    }

    pub async fn query_role_menu(rb: &RBatis, item: QueryRoleMenuReq) -> ServiceResult<QueryRoleMenuData> {
        let menu_list_all = Menu::select_all(rb).await?;

        let mut menu_list: Vec<MenuDataList> = Vec::new();
        let mut menu_ids: Vec<Option<i64>> = Vec::new();

        for y in menu_list_all {
            let x = y.clone();
            menu_list.push(MenuDataList {
                id: x.id,
                parent_id: x.parent_id,
                title: x.menu_name,
                key: y.id.unwrap_or_default().to_string(),
                label: y.menu_name,
                is_penultimate: y.parent_id == Some(2),
            });
            menu_ids.push(x.id)
        }

        if item.role_id != 1 {
            menu_ids.clear();
            let list = sys_role_menu_dao::query_menu_by_role(rb, item.role_id).await?;

            for x in list {
                let m_id = x.get("menu_id").unwrap().clone();
                menu_ids.push(Some(m_id))
            }
        }

        ok_result_data(QueryRoleMenuData { menu_ids, menu_list })
    }

    pub async fn update_role_menu(rb: &RBatis, item: UpdateRoleMenuReq) -> ServiceResult<String> {
        let role_id = item.role_id;

        if role_id == 1 {
            return Err(AppError::BusinessError("不允许操作超级管理员角色"));
        }

        RoleMenu::delete_by_map(rb, value! {"role_id": &item.role_id}).await?;

        let mut role_menu: Vec<RoleMenu> = Vec::new();

        for id in &item.menu_ids {
            let menu_id = id.clone();
            role_menu.push(RoleMenu {
                id: None,
                create_time: Some(DateTime::now()),
                menu_id,
                role_id: role_id.clone(),
            })
        }

        RoleMenu::insert_batch(rb, &role_menu, item.menu_ids.len() as u64).await.map(|_| ok_result_empty())?
    }

    pub async fn query_allocated_list(rb: &RBatis, item: AllocatedListReq) -> ServiceResultPage<UserResp> {
        let page_no = item.page_no;
        let page_size = item.page_size;
        let role_id = item.role_id;
        let mobile = item.mobile.as_deref().unwrap_or_default();
        let user_name = item.user_name.as_deref().unwrap_or_default();

        let page_no = (page_no - 1) * page_size;
        let p = sys_user_dao::select_allocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

        let mut list: Vec<UserResp> = Vec::new();
        for x in p {
            list.push(x.into())
        }

        let total = sys_user_dao::count_allocated_list(rb, role_id, user_name, mobile).await?;
        ok_result_page(list, total)
    }

    pub async fn query_unallocated_list(rb: &RBatis, item: UnallocatedListReq) -> ServiceResultPage<UserResp> {
        let page_no = item.page_no;
        let page_size = item.page_size;
        let role_id = item.role_id;
        let mobile = item.mobile.as_deref().unwrap_or_default();
        let user_name = item.user_name.as_deref().unwrap_or_default();

        let page_no = (page_no - 1) * page_size;
        let d = sys_user_dao::select_unallocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

        let mut list: Vec<UserResp> = Vec::new();
        for x in d {
            list.push(x.into())
        }

        let total = sys_user_dao::count_unallocated_list(rb, role_id, user_name, mobile).await?;
        ok_result_page(list, total)
    }

    pub async fn cancel_auth_user(rb: &RBatis, item: CancelAuthUserReq) -> ServiceResult<String> {
        sys_user_role_dao::delete_user_role_by_role_id_user_id(rb, item.role_id, item.user_id).await.map(|_| ok_result_empty())?
    }

    pub async fn batch_cancel_auth_user(rb: &RBatis, item: CancelAuthUserAllReq) -> ServiceResult<String> {
        SysRoleDao::batch_cancel_auth_user(rb, item.role_id, &item.user_ids).await.map(|_| ok_result_empty())?
    }

    pub async fn batch_auth_user(rb: &RBatis, item: SelectAuthUserAllReq) -> ServiceResult<String> {
        let role_id = item.role_id;

        let mut user_role: Vec<UserRole> = Vec::new();

        for id in &item.user_ids {
            let user_id = id.clone();
            user_role.push(UserRole {
                id: None,
                create_time: Some(DateTime::now()),
                role_id: role_id.clone(),
                user_id,
            })
        }

        UserRole::insert_batch(rb, &user_role, item.user_ids.len() as u64).await.map(|_| ok_result_empty())?
    }
}

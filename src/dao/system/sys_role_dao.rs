use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_role_menu_model::RoleMenu;
use crate::model::system::sys_role_dept_model::RoleDept;
use crate::model::system::sys_role_model::Role;
use crate::vo::system::sys_role_vo::QueryRoleListReq;

pub struct SysRoleDao;

impl SysRoleDao {
    // 封装更新角色状态的数据库操作
    pub async fn update_status(rb: &RBatis, ids: &Vec<i64>, status: i8) -> rbatis::Result<()> {
        let update_sql = format!("update sys_role set status = ? where id in ({})", ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

        let mut param = vec![value!(status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }

    // 封装批量取消用户授权的数据库操作
    pub async fn batch_cancel_auth_user(rb: &RBatis, role_id: i64, user_ids: &Vec<i64>) -> rbatis::Result<()> {
        let update_sql = format!(
            "delete from sys_user_role where role_id = ? and user_id in ({})",
            user_ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(role_id)];
        param.extend(user_ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await.map(|_| ())
    }

    // 封装删除角色及其关联数据的事务操作
    pub async fn delete_role_and_relations(rb: &RBatis, ids: &Vec<i64>) -> rbatis::Result<()> {
        let mut tx = rb.acquire_begin().await?;
        RoleMenu::delete_by_map(&mut tx, value! {"role_id": &ids}).await?;
        RoleDept::delete_by_map(&mut tx, value! {"role_id": &ids}).await?;
        Role::delete_by_map(&mut tx, value! {"id": &ids}).await?;
        tx.commit().await?;
        Ok(())
    }
}


/*
 *角色信息基本操作
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
rbatis::crud!(Role {}, "sys_role");


/*
 *根据id查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_role");

/*
 *根据role_name查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_role_name(role_name:&str) -> Option => "`where role_name = #{role_name} limit 1`"}, "sys_role");

/*
 *根据role_key查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select!(Role{select_by_role_key(role_key:&str) -> Option => "`where role_key = #{role_key} limit 1`"}, "sys_role");

/*
 *分页查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(Role{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_role");

/*
 *根据条件分页查询角色信息
 *author：刘飞华
 *date：2024/12/12 14:41:44
 */
impl_select_page!(Role{select_sys_role_list(req:&QueryRoleListReq) =>"
      where 1=1
     if req.roleName != null && req.roleName != '':
       ` and role_name like concat('%', #{req.roleName}, '%') `
     if req.roleKey != null && req.roleKey != '':
       ` and role_key like concat('%', #{req.roleKey}, '%') `
     if req.status != 2:
       ` and status = #{req.status} `
     if !sql.contains('count'):
        ` order by create_time desc `"},"sys_role");
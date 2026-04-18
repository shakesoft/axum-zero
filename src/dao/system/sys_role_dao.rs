use rbatis::RBatis;
use rbs::value;
use crate::model::system::sys_role_menu_model::RoleMenu;
use crate::model::system::sys_role_dept_model::RoleDept;
use crate::model::system::sys_role_model::Role;

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
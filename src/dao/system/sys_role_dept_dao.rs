


/*
 *角色和部门关联表基本操作
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
use crate::model::system::sys_role_dept_model::RoleDept;

rbatis::crud!(RoleDept {}, "sys_role_dept");

/*
 *根据id查询角色和部门关联表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select!(RoleDept{select_by_id(id:&i64) -> Option => "`where id = #{id} limit 1`"}, "sys_role_dept");

/*
 *分页查询角色和部门关联表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(RoleDept{select_page() =>"
     if !sql.contains('count'):
       order by create_time desc"
},"sys_role_dept");

/*
 *根据条件分页查询角色和部门关联表
 *author：刘飞华
 *date：2024/12/25 10:01:11
 */
impl_select_page!(RoleDept{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       where real_name != #{name}
     if name == '':
       where real_name != ''"
},"sys_role_dept");

use crate::common::error::{AppError, ServiceResult};
use crate::common::result::{ok, ok_result_data};
use crate::dao::system::sys_dept_dao;
use crate::dao::system::sys_dept_dao::SysDeptDao;
use crate::model::system::sys_dept_model::Dept;
use crate::vo::system::sys_dept_vo::{DeleteDeptReq, DeptReq, DeptResp, QueryDeptDetailReq, QueryDeptListReq, UpdateDeptStatusReq};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbatis::RBatis;
use rbs::value;
use validator::Validate;

pub struct SysDeptService;

impl SysDeptService {
    pub async fn add_sys_dept(rb: &RBatis, item: DeptReq) -> ServiceResult {
        if Dept::select_by_dept_name(rb, &item.dept_name, &item.parent_id).await?.is_some() {
            return Err(AppError::BusinessError("部门名称已存在"));
        }

        match Dept::select_by_id(rb, &item.parent_id).await? {
            None => Err(AppError::BusinessError("添加失败,上级部门不存在")),
            Some(dept) => {
                if dept.status == 0 {
                    return Err(AppError::BusinessError("部门停用，不允许添加"));
                }
                let ancestors = format!("{},{}", dept.ancestors.unwrap_or_default(), &item.parent_id);
                let mut sys_dept = Dept::from(item);
                sys_dept.ancestors = Some(ancestors);
                if let Err(e) = sys_dept.validate() {
                    return Err(e.into());
                }
                Dept::insert(rb, &sys_dept).await.map(|_| ok())?
            }
        }
    }

    pub async fn delete_sys_dept(rb: &RBatis, item: DeleteDeptReq) -> ServiceResult {
        if sys_dept_dao::select_dept_count(rb, &item.id).await? > 0 {
            return Err(AppError::BusinessError("存在下级部门,不允许删除"));
        }

        if sys_dept_dao::check_dept_exist_user(rb, &item.id).await? > 0 {
            return Err(AppError::BusinessError("部门存在用户,不允许删除"));
        }

        Dept::delete_by_map(rb, value! {"id": &item.id}).await.map(|_| ok())?
    }

    pub async fn update_sys_dept(rb: &RBatis, mut item: DeptReq) -> ServiceResult {
        let id = item.id;

        if item.id.is_none() {
            return Err(AppError::BusinessError("主键不能为空"));
        }

        if Some(item.parent_id) == id {
            return Err(AppError::BusinessError("上级部门不能是自己"));
        }

        let old_ancestors = match Dept::select_by_id(rb, &id.unwrap_or_default()).await? {
            None => return Err(AppError::BusinessError("部门不存在")),
            Some(dept) => dept.ancestors.unwrap_or_default(),
        };

        let ancestors = match Dept::select_by_id(rb, &item.parent_id).await? {
            None => return Err(AppError::BusinessError("上级部门不存在")),
            Some(dept) => {
                format!("{},{}", dept.ancestors.unwrap_or_default(), &item.parent_id)
            }
        };

        if let Some(dept) = Dept::select_by_dept_name(rb, &item.dept_name, &item.parent_id).await? {
            if dept.id != id {
                return Err(AppError::BusinessError("部门名称已存在"));
            }
        }

        if sys_dept_dao::select_normal_children_dept_by_id(rb, &id.unwrap_or_default()).await? > 0 && item.status == 0 {
            return Err(AppError::BusinessError("该部门包含未停用的子部门"));
        }

        for mut x in sys_dept_dao::select_children_dept_by_id(rb, &id.unwrap_or_default()).await? {
            x.ancestors = Some(x.ancestors.unwrap_or_default().replace(old_ancestors.as_str(), ancestors.as_str()));
            Dept::update_by_map(rb, &x, value! {"id": &x.id}).await?;
        }

        if item.status == 1 && ancestors != "0" {
            SysDeptDao::update_dept_status(rb, &ancestors, item.status).await?;
        }
        item.ancestors = Some(ancestors.clone());

        let data = Dept::from(item);
        if let Err(e) = data.validate() {
            return Err(e.into());
        }
        Dept::update_by_map(rb, &data, value! {"id":  &id}).await.map(|_| ok())?
    }

    pub async fn update_sys_dept_status(rb: &RBatis, item: UpdateDeptStatusReq) -> ServiceResult {
        let mut ids = vec![item.id];
        if item.status == 1 {
            if let Some(x) = Dept::select_by_id(rb, &item.id).await? {
                ids.extend(&x.ancestors.unwrap_or_default().split(",").map(|s| s.i64()).collect::<Vec<i64>>())
            }
        }

        SysDeptDao::update_dept_status(rb, &ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "), item.status)
            .await
            .map(|_| ok())?
    }

    pub async fn query_sys_dept_detail(rb: &RBatis, item: QueryDeptDetailReq) -> ServiceResult<DeptResp> {
        Dept::select_by_id(rb, &item.id).await?.map_or_else(
            || Err(AppError::BusinessError("部门不存在")),
            |x| {
                let data: DeptResp = x.into();
                ok_result_data(data)
            },
        )
    }

    pub async fn query_sys_dept_list(rb: &RBatis, item: QueryDeptListReq) -> ServiceResult<Vec<DeptResp>> {
        Dept::select_page_dept_list(rb, &item)
            .await
            .map(|x| ok_result_data(x.into_iter().map(|x| x.into()).collect::<Vec<DeptResp>>()))?
    }

    //闭包：将全局变量（较大作用域）封闭起来，跟函数一起打包，模拟面向对象封装
    //简称“闭包”，封闭变量，打包进函数，实现变量数据的私有封装，让外部无法修改，只能通过打包的函数进行修改
    //使用闭包模拟类，只不过返回值有点冗长，“行为优先的对象”
    //通过组合（composite）或柯里化（currying），表达函数式编程思想
    pub fn test_closure(num1: i32, num2: i32) -> (impl Fn() -> i32, impl Fn() -> i32, impl Fn() -> i32, impl Fn() -> i32) {
        //1：num1和num2相当于是类的构造函数传入的参数

        //2：num3和num4相当于是类的成员变量
        //闭包的含义解释封闭、包装，就是将这两个成员变量包装成私有性，禁止被外部全局访问读写，只能通过闭包间接访问
        let num3 = 100;
        let num4 = 200;

        //3：返回的闭包相当于是类的成员函数
        return (
            Box::new(move || -> i32 {
                return num1 + num2;
            }),
            Box::new(move || -> i32 {
                return num1 - num2;
            }),
            Box::new(move || -> i32 {
                return num1 * num2;
            }),
            Box::new(move || -> i32 {
                return num1 / num2;
            }),
        );

        //通过闭包模拟轻量的OOP（行为优先），增强型回调函数
        //通过js的一个示例演示
        /*
        let totalScore = 0;
        function calculate_total_score(score)
        {
           //分数是否合规
           if(score<0||score>100)
            return;

           totalScore+=score;
           console.log('计分：'+score);
           console.log('总分：'+totalScore);
        }
        totalScore = 200;//这里容易被改写,造成全局污染
        */

        //使用闭包改写，使用函数包住变量和函数，称为闭包
        /*
        function calculate_total_score(init_score)
        {
            //这行将成员变量封包成私有，不允许外部访问
            let totalScore = init_score;

            return function(score)
            {
               //分数是否合规
               if(score<0||score>100)
                return;

               totalScore+=score;
               console.log('计分：'+score);
               console.log('总分：'+totalScore);
               return totalScore;
            }
        }
        let calculate = calculate_total_score(0);
        console.log(calculate(10));
        console.log(calculate(10));
        console.log(calculate(10));
        */
    }

    pub fn test_closure1() {
        let a = 100;
        let b = || -> i32 {
            let c = 200;
            return a + c;
        };

        let d = || {
            let e = 100;
            return move || -> i32 {
                return b() + a + e;
            };
        };

        ()
    }
}

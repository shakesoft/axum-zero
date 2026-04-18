pub struct SysDeptService;

impl SysDeptService {

    //闭包：将全局变量（较大作用域）封闭起来，跟函数一起打包，模拟面向对象封装
    //简称“闭包”，封闭变量，打包进函数，实现变量数据的私有封装，让外部无法修改，只能通过打包的函数进行修改
    //使用闭包模拟类，只不过返回值有点冗长，“行为优先的对象”
    //通过组合（composite）或柯里化（currying），表达函数式编程思想
    pub fn test_closure(num1:i32,num2:i32)->(impl Fn() -> i32,impl Fn() -> i32,impl Fn() -> i32,impl Fn() -> i32) {
        //1：num1和num2相当于是类的构造函数传入的参数

        //2：num3和num4相当于是类的成员变量
        //闭包的含义解释封闭、包装，就是将这两个成员变量包装成私有性，禁止被外部全局访问读写，只能通过闭包间接访问
        let num3 =100;
        let num4 =200;

        //3：返回的闭包相当于是类的成员函数
        return (Box::new(move||->i32 {
            return num1+num2;
        }),Box::new(move||->i32 {
            return num1-num2;
        }),Box::new(move||->i32 {
            return num1*num2;
        }),Box::new(move||->i32 {
            return num1/num2;
        }));

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

    pub fn test_closure1()
    {
        let a = 100;
        let b = ||->i32{
            let c = 200;
            return a+c;
        };

        let d =||{
            let e = 100;
            return move||->i32{
                return b()+a+e;
            };
        };

        ()
    }
}
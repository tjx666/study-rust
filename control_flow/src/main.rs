/*
很多编程语言的教程都是类似的顺序
- 变量声明和定义
- 注释
- 数据类型
- 操作符
- 流程控制
- 函数
- 类等复杂类型
- 文件 io
- http
 */

#[allow(unused)]

fn main() {
    // rust 是基于表达式的语言，这意味 rust 的世界是由表达式组成，语句也是为表达式服务的
    // 块（特意不叫语句块）也是表达式
    let result = {
        let a = 1;
        a + 5
    };
    println!("{}", result); // 6

    // 不知道在哪听说的，一门编程语言只要实现了条件，循环语句，以及 goto，就能在有线的事件计算出一切能计算出的任务
    // 撸了好几年代码，感觉确实好像流程控制只需要条件和循环，以及 break 之类的调出当前执行的控制流就够了

    // ------------------------ 条件表达式 -----------------------------
    let a = 1;
    let b = 2;
    // if 后面不需要加括号，当然你要是加了也不会报错
    if a > b {
        println!("a is bigger than b");
    }
    // 紧跟着 if/else 的块表达式叫做分支
    else {
        println!("a is not bigger than b");
    }

    // 条件语句可能有多分支
    let num = 9;
    if num % 2 == 0 {
        println!("6 can be divided by 2");
    } else if num % 3 == 0 {
        println!("6 can be divided by 3");
    } else if num % 4 == 0 {
        println!("6 can be divided by 4");
    } else {
        println!("6 can't be divided by 2, 4, 3");
    }

    // 不像其它编程语言 if 语句中分支代码如果只有 1 行，一般不加花括号，rust 中 if/else 后面必须加花括号
    // if num % 5 == 0 println!("num can be divided by 5") else println!("777")

    // 在 rust 中叫条件表达式，而不是条件语句，这意味着你可以将 if 表达式用于赋值
    // 其它语言里面有三目运算符 ?，但是经常被滥用导致代码难以理解
    let condition = false;
    let s = if condition { 5 } else { 9 };

    // ------------------------ loop 循环 -----------------------------
    // 记得大学有个编程大作业是用 c 语言实现图书管理系统
    // 实现用户登入的时候需要让用户循环输入密码，这个操作是必定会执行一次的，适合用 loop 循环
    // 简言之：loop 循环适合在必须循环一次的情况下使用
    // loop {
    //     println!("print again!");
    // }

    let mut d = 1;
    let x: i32 = loop {
        d += 1;
        if d == 10 {
            // break 返回循环表达式的值
            break d * 2;
        }
    };

    // 循环标签，前多层循环时会比较有用，可以直接从里层循环往外跳出好几层
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // ------------------------ while 循环 -----------------------------
    // 如果每次循环都要先判断条件再循环，比较适合用 while 循环
    // 所有循环其实都可以用 while 循环和 loop 循环改写
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // ------------------------ 集合循环- for 循环 -----------------------------
    let day = ['日', '一', '二', '三', '四', '五', '六'];
    for d in day {
        println!("星期{}", d);
    }

    // 可以使用 Range 来遍遍历特定次数
    for i in (1..10) {
        println!("{}", i);
    }

    // 反向遍历
    for i in (1..4).rev() {
        println!("{}", i);
    }

    // 包括上边界
    for i in (1..=3) {
        println!("{}", i);
    }

    // step
    for i in (2..=10).step_by(2) {
        println!("{}", i);
    }

    // 如果循环退出的条件较为复杂，不是简单的大于小于某个数字，不适合用 for 循环

    // for 循环标签
    'for_loop_outer: for i in (1..=10) {
        for j in (1..11).rev() {
            println!("{}, {}", i, j);
            if (i == 2 && j == 1) {
                break 'for_loop_outer;
            }
        }
    }
}

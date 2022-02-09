/*
 定义一个函数，python 用 def, js 用 function, go 语言用 func，rust 用 fn，可见 rust 很简洁
*/

#[allow(unused)]

fn main() {
    println!("Hello, world!");
    // 不像 c/c++, 类似于 js，函数的声明不一定要在使用之前
    another_function();

    {
        fn inner_func() {}
    }

    // 当然只能访问同层或者外层作用域声明的函数
    // inner_func();

    print_labeled_measurement(5, 'h');

    // ------------------------ Statement and Expression -----------------------------
    // rust 是一个基于表达式的语言
    // 函数声明是语句，没有返回值
    // let a = fn tt() {};

    // let m = 6 是语句，语句不能用于赋值表达式
    // let n = (let m = 6);
    let mut j = 6;
    let mut i = 9;
    // expected integer, found `()`
    // 和其它语言不一样，j = 8 返回的是 ()，i 是 i32 类型，类型不兼容
    // i = j = 8;
    // k 是单位类型，因为 i = 9 返回的是单位值
    let k = i = 9;

    // ------------------------ 返回值 -----------------------------
    // 函数的返回值是函数体最后一个表达式，如果最后的是语句，那么返回的是 ()
    fn get_five() -> i32 {
        5
    }

    fn get_five_plus_x(x: i32) -> i32 {
        // 如果是 5 + x; 那返回的就是 ()
        5 + x
    }

    let num = get_five_plus_x(1);
    println!("return value is {}", get_five());

    // 不写函数的返回值类型等同于声明返回类型是单位类型，并不像 ts 中会进行类型推导
    // a +b 返回是 i32, 但函数声明的类型是 ()
    // fn add(a: i32, b: i32) {
    //     a + b
    // }
}

fn another_function() {
    println!("Another function.");
}

// 带参数的函数，rust 的类型声明和 ts 很像，都是后置的
// 参数的类型必须声明
fn print_labeled_measurement(value: i32, label: char) {
    println!("The measurement is {}{}.", value, label);
}

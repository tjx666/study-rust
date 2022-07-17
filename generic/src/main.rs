#![allow(dead_code)]
#![allow(unused_variables)]

/**
 * 有经验的程序员会自觉的对代码抽象复用
 * 一般步骤：
 * 1. 发现重复代码
 * 2. 将重复代码封装成函数
 * 3. 将不同的地方抽象参数，不同的类型可以使用泛型，变量则使用参数
 */

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct List<T> {
    elements: Vec<T>,
}

impl<T> List<T> {
    fn hello() {
        println!("hello");
    }
}

// 和其它语言例如 java, c++, ts 泛型没啥太大差别
// 其中一个不太一样的就是 rust 允许为泛型的实例类型声明方法
impl List<i32> {
    fn add(&self, other: i32) -> i32 {
        return self.elements[0] + other;
    }
}

// 为什么 rust 的泛型没有运行时开销？
/*
因为编译结束后 rust 为每一个泛型的实例类型声明了具体的类型
因此，可以预料的是使用的泛型的实例类型越多，编译的体积会越大。
 */

 // 编译完，会生成 List_f32 类型
fn test1(a: List<f32>) {
    println!("{}", 6);
}

fn test2(a: List<f64>) {
    println!("{}", 6);
}

fn test3(a: List<u8>) {
    println!("{}", 6);
}

fn test4(a: List<u32>) {
    println!("{}", 6);
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

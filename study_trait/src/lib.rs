#![allow(dead_code)]
#![allow(unused_variables)]

// use std::fmt::Display;

use std::fmt::{Debug, Display};

// 定义 trait 和定义结构体，枚举没啥区别
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认实现可以调用其他方法
    // 默认实现的意义
    // 1. 当所有实现该 trait 的类型该方法的实现完全一样，这一层意义才是最有价值的，可以让我们少些很多代码
    // 2. 提供默认实现，只是为了方便通过编译
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait WithSummarize {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl WithSummarize for Tweet {
    fn summarize(&self) -> String {
        format!("WithSummary {}: {}", self.username, self.content)
    }
}

// 为什么不允许为外部类型实现外部 trait
// 反证法：
/*
例如有第三方库 pkg1, pkg2
当你在 pkg1 为 Vec 声明 Display trait
并且在 pkg2 为 vec 声明 Display trait
这时存在两个 fmt 的实现

那么现导入 Vec 和 Display, rust 并不能确定到底使用哪一个 fmt 的实现
*/

/* 报错
only traits defined in the current crate can be implemented for types defined outside of the crate
define and implement a trait or new type instead
*/
// impl Display for Vec<i32> {

// }

// 其实很多语言都有 trait 的类似概念，例如 java 和 ts 的 interface
// 不过它们之中的 interface 都运行抽象公共属性，这在 rust 不行

// ------------------------ trait 作为参数和返回值 -----------------------------
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 返回值使用 trait 只能用于函数体只返回一种类型的情况
// 下面的函数可能返回 NewsArticle 也可能返回 Tweet
/* fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
} */

// ------------------------ trait bounds -----------------------------
// 类似于 ts 中对泛型使用 extends 对泛型做约束，在 rust 中叫做 trait bounds
// impl trait 其实是 trait bounds 的语法糖
fn notify1<T: Summary>(article: T) {
    println!("{}", article.summarize());
}

// 使用 impl 的局限性：无法限制各个类型之间的关系，例如这里不能保证 a, b 是同一个类型
fn join1(a: impl ToString, b: impl ToString) -> String {
    a.to_string() + &b.to_string()
}

fn join2<T: ToString>(a: T, b: T) -> String {
    a.to_string() + &b.to_string()
}

// 泛型可以有多个边界约束，使用 + 拼接
trait Singer {
    fn sign() {}
}

trait Dancer {
    fn dance() {}
}

// 使用 impl 语法糖在简单情况下确实比 trait bounds 更简洁
fn show1(person: impl Singer + Dancer) {}
fn show2<T: Singer + Dancer>(person: T) {}

// trait bounds 比较复杂的时候签名看着就不那么清晰
fn complex_fn1<M: Display + Debug, N: Copy + Debug>(a: M, b: N) -> N {
    b
}

// 使用 where 从句来声明可以让代码看起来更简洁清晰
fn complex_fn2<M, N>(a: M, b: N) -> N
where
    M: Display + Debug,
    N: Copy + Debug,
{
    b
}

// 将 trait bounds 和 结构体 impl 结合
// 可以只给泛型满足 trait bounds 的类型实现方法

struct Student<T> {
    a: T,
    b: T,
}

// 只为实现了 Debug trait 的 Student 类型实现方法
impl<T: Debug> Student<T> {
    fn study() {}
}

impl<T: Display> Summary for T {
    fn summarize_author(&self) -> String {
        "s".to_string()
    }
}

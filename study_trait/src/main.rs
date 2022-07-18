#![allow(dead_code)]
#![allow(unused_variables)]

// 为什么使用 trait 上的方法需要导入 trait ？
// 因为可以对同一个类型实现不同的 trait 并且这些 trait 有相同的方法签名
// 这时编译器就不知道实现的是哪个 trait 了
// 李如意下面如果还导入 WithSummarize 就会报不知道使用哪个 trait
use study_trait::{Summary, Tweet};

// 定义 trait 也可以使用泛型
trait TrailWithGeneric<T> {
    fn play(t: T) {}
}

struct StructExample<T> {
    a: T,
    b: T,
}

impl<T> TrailWithGeneric<T> for StructExample<T> {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

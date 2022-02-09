/*
入行越久，越感到必要的注释是非常重要的，每个人都致力于写出令人容易理解的代码
但是不要高估自己觉得代码别人肯定能看懂，看不懂都是技术不到位...
有时候碰到值得注意，踩过坑，或者非常复杂，难以理解的代码，如果不加注释会有很多问题
- 不像你在上学的时候，老师会经常带你复习以前的内容，一份代码隔个一两天不看，你可能就会忘了当初那样设计的原因
- 同事切入到同一份代码，不能立马理解代码意图，需要向你咨询，浪费各自时间
- 注释就是文档，文档是个人，是公司的技术沉淀，是财富
- 你难道不喜欢一个注释写的很规范的同事吗？
 */

#[allow(unused)]

fn main() {
    // 单行注释，没什么稀奇的
    // output hello world
    println!("Hello, world!");

    /*
    这是多行注释，和其它语言也一样没什么稀奇
    不能嵌套
    */

    let a /* 嵌入到语句里面也没关系*/ = 666;
    let b = 1 + /* lala */ 1;
}

///
/// 这是文档注释
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/**
 * 顺斜杆加两个 * 开头的多行注释也会被理解为文档注释，但是显示效果上和 /// 的文档注释有区别
 * 这里虽然是换行写了，但是使用 cargo doc 生成的 html 页面中，文档内容还是单行的
 */
fn test() {}

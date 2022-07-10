#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    // 回顾下数组和元组

    // 数组
    let list = [1, 2, 3];
    for n in list {
        print!("{}", n);
    }
    println!("");

    // 元组
    let tuple = (1, 'a', "hello");
    println!("{}", tuple.2);

    // ------------------------ vector -----------------------------
    /*
    数组和元组共同特点：
    1. 数据在内存中是连续的
    2. 长度不可变
    3. 存放在栈上，当然了，存放在栈上的变量都是不可变的

    向量和它们的不同点：
    1. 存放在堆上
    2. 长度可变
     */

    // 创建 vector
    // vector 也是一个结构体
    let v = vec![1, 2, 3];

    // 无法自动推导出类型就手动标
    let v1: Vec<i32> = Vec::new();

    // 这里元素类型是根据下一行推断出来的，从这里可以看出类型推导比 ts 强
    let mut v2 = Vec::new();
    v2.push(1);

    // 访问 vector
    // 使用下标访问
    println!("{}", v[0]);
    // 越界 panic
    // 'index out of bounds: the len is 3 but the index is 10'
    // println!("{}", v[10]);

    // 使用 get 方法访问
    match v.get(10) {
        Some(n) => println!("{}", n),
        None => println!("越界访问"),
    }

    // 更新 vector
    let mut v3 = vec![1];
    v3.push(2);
    // 和上面一行换一下就会报错了，不能同时存在可变引用和不可变引用
    let member = &v3[0];
    println!("{}", member);
    v3.pop();

    // 遍历 vector
    for i in &v3 {
        println!("{}", i);
    }

    for i in &mut v3 {
        *i += 2;
    }

    // ------------------------ String -----------------------------
    // 创建字符串
    let str1 = String::from("hello world");
    let mut str1 = "hello world".to_string();
    let mut num_str = 123.to_string();
    println!("{}", num_str);

    // 更新字符串
    str1.push('!');
    num_str.push_str("34567");
    let hello = "hello".to_string();
    let world = "world".to_string();
    let hello_world = hello + &world;
    // hello 的所有权已经在 add 被转移了
    // println!("{}", hello);

    let s = format!("{}: {}", "name", "ly");
    println!("{}", s);

    // 字符串标量
    for c in "🚀".chars() {
        println!("{}", c);
    }
    // 字符串 u8 字节
    // 4 字节字符
    for b in "🚀".bytes() {
        println!("{}", b);
    }

    // ------------------------ HashMap -----------------------------
    // 其它语言中的 Map, Dict
    // 创建
    let mut map = HashMap::new();
    map.insert("name", "ly");

    let vec1 = vec!["name", "hobby"];
    let vec2 = vec!["ly", "game"];
    let m2 = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .collect::<HashMap<_, _>>();

    // 访问
    match m2.get("name") {
        Some(s) => println!("{}", s),
        None => println!("{}", "none"),
    }

    // 更新
    map.entry("name").or_insert("lily");
    let nums = vec![1, 1, 2, 2, 3, 3, 3, 4, 4];
    let mut count = HashMap::new();
    for i in &nums {
        let v = count.entry(i).or_insert(0);
        *v += 1;
    }
    println!("{:?}", count);
}

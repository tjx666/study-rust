#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Read;

fn read_file1() -> File {
    let path = "/Users/yutengjing/Desktop/abc.txt";

    match File::open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(f) => f,
                Err(_) => panic!("文件创建失败！"),
            },
            _ => panic!("打开文件发生为止异常！"),
        },
    }
}

fn read_file2() -> Result<File, IoError> {
    let path = "/Users/yutengjing/Desktop/abc.txt";
    let file = File::open(path)?;
    Ok(file)
}

fn read_file3() -> Result<String, IoError> {
    let path = "/Users/yutengjing/Desktop/abc.txt";
    let mut file = File::open(path)?;
    let mut s = String::from("");
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file4() -> Result<char, IoError> {
    let path = "/Users/yutengjing/Desktop/abc.txt";
    let mut file = File::open(path)?;
    let mut s = String::from("");
    file.read_to_string(&mut s)?;
    s.lines()
        .next()
        .ok_or(IoError::new(ErrorKind::Other, "文件为空"))?
        .chars()
        .last()
        .ok_or(IoError::new(ErrorKind::Other, "文件为空"))
}

fn main() -> Result<(), IoError> {
    // panic!("触发恐慌");

    // let v = vec![1, 2, 3];
    // v[99];

    read_file1();

    // Result 类型线上的方法
    // unwrap 打开的意思，可以简化处理 Result 是错误的情况
    // let f = File::open("hello.txt").unwrap();
    // expect 类似 unwrap，但是可以给错误提供更友好的提示
    // 从这可以看出 rust 应该没有函数重载
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f1 = read_file2()?;

    Err(IoError::new(ErrorKind::Other, ""))
}

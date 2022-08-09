#![allow(dead_code)]
#![allow(unused_variables)]

// 存储值类型的 List，类型无限递归，无法计算 List 大小
// enum List {
//     Cons(i32, List),
//     Nil,
// }

use std::{cell::RefCell, rc::Rc, borrow::Borrow};
use List::*;

enum List {
    // 使用间接性的指针
    Cons(i32, Rc<List>),
    Nil,
}

struct Test {
    n: i32,
}

fn main() {
    let t = Test { n: 10 };
    let t1 = t;
    // println!("{}", t.n);

    let s = String::from("");
    let s1 = s;

    let i = Box::new(1);
    println!("{}", *i + 2);
    // explicit destructor calls not allowed
    // i.drop();
    drop(i);
    // println!("{}", i);

    let data = vec![1, 2];
    let v: &[i32] = &data[1..2];
    println!("{:?}", v);
    let vv: Vec<i32> = v.to_owned();
    println!("{:?}", vv);

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    println!(
        "count after creating b = {}",
        match b {
            Cons(i, rf) => Rc::strong_count(&rf),
            Nil => 1,
        }
    );
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let t1 = Test { n: 1 };
    let ref_cell = RefCell::new(t1);
    (*ref_cell.borrow_mut()).n = 2;
    println!("{}", ref_cell.borrow().n);
}

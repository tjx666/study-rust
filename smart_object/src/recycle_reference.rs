use std::{cell::RefCell, rc::Rc};

enum List {
    // 使用间接性的指针
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test() {
        let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));
        let b = Rc::new(Cons(2, RefCell::new(Rc::clone(&a))));
        if let Cons(_, next) = &*a {
            *next.borrow_mut() = Rc::clone(&b);
        }
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(&b));
    }
}

struct A {
    s: String
}


fn t1() {
    let a = A{s: String::from("")};
    let b = a.s;
    // println!("{}", a.s);
    let a = String::from("");
    let b = a;
}

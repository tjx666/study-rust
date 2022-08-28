// 使用泛型约束返回值类型和参数类型必须相同
fn get_self<T>(a: T) -> T {
    a
}

fn join_string<'a>(s1: &'a mut String, s2: &String) -> &'a String {
    s1.push_str(s2);
    s1
}

fn test_fn() {
    let mut v = vec![1, 2, 3];
    let closure = || {
        println!("{:?}", v);
    };

    fn pass_fn<F: FnMut() -> ()>(f: F) {

    }

    pass_fn(closure);
}

fn main() {
    join_string(&mut String::from("a"), &String::from("b"));
    println!("Hello, world!");
}

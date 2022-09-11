// 编译完这个函数就无了
fn get_self<T>(v: T) -> T {
    v
}

fn main() {
    let a = 1;
    get_self(a);
    // 因为使用了 i32 作为泛型类型所以编译完生成下面的函数代码
    /*
    fn get_self_i32(v: i32)-> i32 {
        v
    }
    */
}

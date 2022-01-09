fn main() {
    // 声明可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    // 声明不可变变量
    let double_x = x * 2;
    println!("The value of x is: {}", double_x);

    // 声明常量
    // 声明常量时必须标注类型
    // const 其实是编译器的概念，运行时就没 const 了
    // 常量的作用:
    // 1. 处理魔法数字
    // 2. 便于重构
    const PI: f32 = 3.14159265457;
    const RADIUS: f32 = 10.0;
    // 和 dart, c++ 一样，常量的赋值只能是常量表达式
    const AREA: f32 = PI * RADIUS * RADIUS;
    println!("{}", AREA);

    // 变量覆盖
    let v = 3;
    // 同一作用域，后面声明同名变量，会覆盖前面声明的变量
    // 可以使用这种方式改变变量的可变性
    let mut v = v + 1;
    v += 1;

    {
        // 子作用域覆盖父作用域同名变量
        let v = v * 2;
        println!("{}", v);
    }

    println!("{}", v);

    // 变量覆盖还可以是不同类型
    let v = "lalala";
    println!("{}", v);

    // 还有一种应用场景, 可能你会想声明一个不可变变量，但是需要经过多次计算
    // 所以你会先声明是可变的去计算，最后使用不可变方式再声明一次
    let mut s = 1;
    s += 1;
    s *= 2;
    let s = s;
    println!("{}", s);
}

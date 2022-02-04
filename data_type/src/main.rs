#[allow(unused)]

fn main() {
    // rust 数据类型分 scalar(标量类型) 和 compound(复合类型) 两种类型

    // scalar 分 integer, float, boolean, char，和 c++/java 差不多

    // integer 分为有符号和无符号类型，分别使用 i 和 u 前缀区分
    // integer 类型由符号 + bit 位数组合而成，例如 i8, i32, u64

    // 有符号数的范围 -2 ** (n - 1) <= x <= 2 ** (n - 1) - 1, 无符号数范围 0 <= x < z ** n - 1, n 为 bit 位数
    // 和 js 一样，rust integer 类型可以有各种进制表示法
    let hex_num: isize = 0x10;
    let octal_num: u8 = 0o10;
    // 可以把类型写在字面量后面
    let one = 0b1u8;
    // 可以用数字分隔符
    let large_num = 1000_000i32;
    // 还可以用字符表示，只能是 u8 类型
    let char_num = b'A';
    // size 和 usize 类型大小和 cpu 架构有关，因此一起叫做 arch 类型
    // 32 机器 size 大小为 32 位，64 位机器大小为 64 位
    // 和 c++ 中 size_t 差不多

    // 默认为 i32 类型
    let num = 256;

    // 直接声明大于 i32 的数编译报错
    // let big_num = 666666666666;

    // 像下面这样操作 num 会导致 num 被推导成 i64
    // let big_num: i64 = 666666666666;
    // num += big_num;

    // ------------------------ 浮点数 -----------------------------
    // 和很多其它编程语言一样，rust 也使用 IEEE754 标准
    // rust 浮点数有 f32 和 f64 两种类型，默认类型推导是 f64
    let float1 = 3.14;

    // ------------------------ 数字运算符 -----------------------------
    // 加法
    let sum = 1 + 2;
    // 减法
    let difference = 9 - 6;
    // 乘法
    let product = 2 * 10;
    // 除法
    let quotient = 10 / 5;
    // integer 除 integer 结果还是 integer, 会向下取整
    let floored = 10 / 3; // 3
                          // 求模
    let remained = 10 % 3;
    // 求幂
    println!("2 ** 3: {}", i32::pow(2, 3));
    println!("2 的 10 次幂是：{}", 2i32.pow(10));

    // ------------------------ 布尔类型 -----------------------------
    // 占用 1 个字节存储空间
    // 自动推导出 bool 类型
    let t = true;
    // 像 ts 就用 boolean，rust 用用 bool 少几个单词
    let f: bool = false;

    // ------------------------ 字符类型 -----------------------------
    // 和 c/c++ 不一样的是， rust 中一个字符使用 4 个字节存储
    // 所以 rust 的字符能表示中文字符，emoji 等 4 字节字符
    let hook = '✅';

    // ------------------------ 复合类型 compound type -----------------------------
    // rust 有两种原生的复合类型：元组和数组

    // ------------------------ 元组 -----------------------------
    // 和 python, ts 中的元组类似, rust 的元组可以理解为元素类型可以是多种，长度固定的数组
    // () 属于单位类型(unit type)，只有一个值也就是 ()，称作单位值
    let unit = ();
    let coordinate = (0, 0);
    // 声明指定元组类型
    let coordinate3d: (f64, f64, f64) = (3.14159, 8.75, 1.735);
    // 元素可以是不同类型
    let identifier = ("YuTengjing", 25);
    // 类似于 ES6 的解构赋值
    let (first, second) = (1, 2);
    println!("{}", coordinate3d.1); // => 8.75

    // ------------------------ 数组 -----------------------------
    // 类似于 c++ 的数组，rust 的数组也是元素类型必须相同，长度固定
    // rust 声明数组的方式和 js 差不多，比 c++ 优雅太多了
    let arr = ['a', 'b', 'c'];
    let day = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    // 声明数组类型，数组的类型又两部分组成，元素类型和数组长度
    let parts_of_day: [&str; 2] = ["AM", "PM"];
    // 声明时不能省略数组长度
    let sexes: [char; 2] = ['男', '女'];
    // rust 初始化数组的便捷性就可以看出 rust 是一门高级语言，有很多实用的语法
    let mut directions = ['东'; 4];
    directions[1] = '南';
    directions[2] = '西';
    directions[3] = '北';
    // 编译时就检测出来了
    // directions[4] = 'x';
    // let error_index = 4;
    // 编译期间还是检测出来了
    // directions[error_index] = 'y';
    // let computed_index = "东南西北".len();
    // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 12'
    // 不像 c++，越界错误会被视为合理的内存访问
    // directions[computed_index] = 'z';
}

fn main() {
    // rust 数据类型分 scalar 和 compound 两种类型

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
    println!(
        "{} {} {} {} {}",
        hex_num, octal_num, one, large_num, char_num
    );

    // 默认为 i32 类型
    let num = 256;
    println!("{}", num);

    // 直接声明大于 i32 的数编译报错
    // let big_num = 666666666666;
    // println!("{}", big_num);

    // 像下面这样操作 num 会导致 num 被推导成 i64
    // let big_num: i64 = 666666666666;
    // num += big_num;
    // println!("{}", num);
}

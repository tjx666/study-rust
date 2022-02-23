/*
由于计算机的内存是有限的，因此一个程序需要管理好内存，避免资源占用过高和内存泄漏

有些语言是有 gc 管理内存，例如 java/go
优点：程序员不用操行内存管理，适合写业务系统
缺点:
- 本身比较吃内存
- 在运行 gc 会导致延迟
- gc 触发的时机不可预测
- 内存可能不会在第一时间被释放，那么内存利用率就不高

有些语言需要程序编写者自己管理内存释放，例如 c/c++
优点：可以对程序的内存管理有最精细的控制，适合编写对性能要求较高的模块
缺点:
- 经常会犯忘记释放内存，对一块内存重复释放，访问空指针等问题，不过 c++ 的智能指针可以一定程度缓解这些问题
- 一旦发生错误的内存访问和内存泄漏，对于应用和系统可能是致命的
- 手动管理内存太费脑

rust 没有采用 gc, 也没有完全把管理内存的责任丢给程序员，而是采用了另外的方式：所有权机制
rust 最大语言特色就是所有权机制，运行时零开销，编译期间就能检查出内存安全问题
*/

#[allow(unused)]

fn main() {
    // ------------------------ 栈内存与堆内存 -----------------------------
    /*
    存放内容：
    栈内存：用于存放大小已知且大小固定的值，后进先出
    堆内存：用于存放大小未知或大小不固定的值

    内存中的存在形式:
     - 栈内存值是连续存放的，后进先出
     - 堆内存在内存是不连续，散乱的，访问一个堆内存的值可能需要在内存中多次跳转访问

    分配内存速度：
     - 栈更快，因为就是放栈顶，位置固定
     - 堆更慢：因为需要先向系统申请内存中没有被使用的一块大小足够的内存，标记为已使用，再清理那块内存

    访问速度：
     - 栈内存更快，因为内存连续
     - 堆内存更慢，因为可能需要多次跳转
    */

    // ------------------------ 所有权规则 -----------------------------
    // Each value in Rust has a variable that called it's owner
    // There can only be one owner at a time
    // When the owner goes out of scoop, the value will be dropped

    // ------------------------ str 和 String 类型 -----------------------------
    // 字符串字面量是编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中，内容和大小都不可变的，类型名是 str
    let str = "字符串字面量";

    // String 类型内存是内容和大小都可变的，放在队内存上
    let string = String::from("String 类型实例");

    // ------------------------ 移动 -----------------------------
    // 声明不可变变量 x，并使用 i32 类型的 5 初始化 x，将 5 压入栈内存
    let x = 5;
    // 复制 x 指向的栈内存，再压入栈内存
    let y = x;

    // 申请了栈内存存放 s1 的成员
    // 申请堆内存存放 s1.ptr 指向的字符串字节
    // s1 的长度表示的是字符串内容的长度，容量表示申请的内存大小，和 c++ 的 vector 一样，容量是会动态调节的
    let s1 = String::from("hello");
    // 在 js, java 等语言我们会称之为浅拷贝，只是引用复制，指向的内存没有被复制
    // 不同的是由于所有权规则第二条，一个时刻一个值只有一个 owner，这里的 s1 其实会失效，s2 变成 owner
    // 这种行为叫 move，move 其实还确保了多个变量指向同一块内存，离开作用域时不会多次释放
    let s2 = s1;
    // 报错：借用了已经被移动的变量
    // println!("{}", s1);

    // ------------------------ 克隆 -----------------------------
    // 其实也就是深拷贝
    let s3 = s2.clone();
    // 深拷贝后还可以继续使用 s2
    println!("{}", s2);

    let tup1 = (0, 0);
    let tup2 = tup1;
    // 还是可以访问 tup1，因为 (i32, i32) 类型实现了 copy trait
    println!("{}", tup1.1);
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    // 任何简单的标量和标量的组合都实现了 copy trait

    let str_tup1 = (1, String::from("666"));
    let str_tup2 = str_tup1;
    // 报错，str_tup1 实现了 drop trait 所以不能实现 copy trait
    // println!("{}", str_tup1.0);

    // ------------------------ 所有权与函数 -----------------------------
    // 函数传参其实和赋值是一样的行为，可能会发生赋值或者移动
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let a = 1;
    let b = 2;
    // 存放栈上的变量 a, b 都是发生赋值
    add(a, b);

    fn print_length(s: String) {
        println!("{}", s);
    }

    let ss = String::from("ss");
    print_length(ss);
    // 报错，ss 在上面传参的时候已经发生了移动
    // println!("{}", ss);

    // 函数返回值也可以发生移动
    fn borrow_then_back(s: String) -> String {
        s
    }

    let ss1 = String::from("ss1");
    // 这里发生了啥？
    // 1. 将 ss1 所有权转移给了 函数参数 s, ss1 失效
    // 2. 通过函数返回值把借出去的所有权还回给了 ss2
    let ss2 = borrow_then_back(ss1);
    // borrow of moved value: `ss1` value borrowed here after move
    // println!("{}", ss1);

    // 我们可以使用元组来返回多个值
    fn get_origin() -> (i32, i32) {
        (0, 0)
    }

    // ------------------------ 引用和借用 -----------------------------
    fn calculate_len(s: &String) -> usize {
        // 声明一个引用类型就是 &Type
        // s 引用传进来的 s3 并没有转移所有权
        s.len()
    } // s 离开作用域只会弹出栈顶的 s 引用本身，不会回收 s3, 因为 s 并不是 s3 的 owner
    let s3 = String::from("s3");
    calculate_len(&s3);
    // 可以继续使用 s3, owner 还是 s3
    println!("{}", s3);

    // 引用是名词，借用是动词
    // We call the action of creating a reference borrowing

    //  &Type 声明的是不可变引用
    fn change_1(s: &String) {
        // `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        // s.push_str(", world!");
    }

    fn change_2(s: &mut String) {
        s.push_str(", world!");
    }
    let mut hello = String::from("hello");
    change_2(&mut hello);
    println!("{}", hello);

    let s4 = String::from("s4");
    // 一个变量可以有无数个不可变引用
    let r1 = &s4;
    let r2 = &s4;

    // rust 不允许同时使用多个可变引用
    let mut s5 = String::from("s5");
    let mutable_r1 = &mut s5;
    let mutable_r2 = &mut s5;
    // 用了就会报错：cannot borrow `s5` as mutable more than once at a time
    // println!("{}", mutable_r1);

    // rust 不允许同时使用指向同一个变量的不可变引用和可变引用
    let mut s6 = String::from("s6");
    let s6_ref = &s6;
    let s6_mutable_ref1 = &mut s6;
    // 同时使用可变和不可变引用可能会导致数据竞争
    // println!("{}{}",s6_ref,  s6_mutable_ref1);

    // 不允许同时使用多个可变引用或者同时同时可变与不可变引用的原因：rust 想要在编译期间就能避免发生数据竞争
    // 我理解数据竞争应该就是并发安全的意思

    // 数据竞争发生的条件
    // 1. 同时存在 1 个以上的引用
    // 2. 至少有一个变量有权访问变量
    // 3. 没有其它机制用于管理同步访问（例如 java 中可以用锁）

    // 可以通过创建一个 scoop 来使用多个可变引用
    {
        // 这种情况明显不会发生数据竞争，同步执行代码
        let s6_mutable_ref2 = &mut s6;
        println!("{}", s6);
    }

    // 引用的作用域范围是声明引用的地方到最后一次使用它的地方
    let mut good = String::from("好");
    let good_ref = &good;
    println!("{}", good_ref); // 最后一次使用 good_ref 在这，作用域结束

    let good_mut_ref = &mut good;
    println!("{}", good);

    // rust 编译器能够保证引用是可用的，不会出现像 c++ 中的悬空指针或者说野指针
    // fn get_ref() -> &String {
    //     let s = String::form("xx");
    //     &s
    // } // s 出了作用域就销毁了, 返回的引用就悬空了
    // 这里可以直接返回 s，通过返回值转移所有权
    // his function's return type contains a borrowed value, but there is no value for it to be borrowed from

    // ------------------------ 切片 -----------------------------

    println!("{}", "我".as_bytes().len()); // 3
    println!("{}", "a".as_bytes().len()); // 1
    println!("{}", "🚀".as_bytes().len()); // 4

    /// 返回字符串第一个单词
    /// 可以认为单词前面没有空格，返回第一个单词所在下标 + 1
    fn find_word(s: &String) -> usize {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if (item == b' ') {
                return i;
            }
        }
        return s.len();
    }
    // 返回下标的方式其实不太安全，因为可能字符串内容被修改，索引失效

    let s7 = String::from("Hello world!");
    println!("{}", find_word(&s7));

    let s8 = String::from("Pine apple");
    // 切片本质就是引用，只不过带范围
    let slice = &s8[..];

    // 返回切片避免字符串被修改
    fn first_word1(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    {
        let mut s = String::from("hello world");

        first_word1(&s);

        s.clear(); // 错误!

        // println!("the first word is: {}", word);
    }

    // 字符串字面量类型本质上是一个切片，不可变引用，所以不可修改
    // 使用字符串切片做类型参数比使用 &String 更通用
    // 当你吧 &String 传给字符串切片参数时发生了类型转换

    // 很多集合类型都有切片
    let arr = [1, 2, 3];
    let arr_slice = &arr[..];

    /*
    总的来说：
    1. 所有权机制保证了变量在脱离了作用域后内存被回收
    2. 引用限制保证了不会发生数据竞争和访问悬空指针
    3. 切片工具让我们在数据安全访问更进一步，可以访问集合的一部分，并且不会发生数据竞争
     */
}

/*
枚举

rust 的枚举和我学过的 java, ts 中的枚举差别很大
rust 中的枚举每个成员的结构居然可以不一样
 */

#[allow(unused)]

fn main() {
    // ------------------------ 定义枚举 -----------------------------
    // 最简单的枚举定义的方式和 ts 一样
    enum Season {
        // 其实这里相当于定义了四个单位结构体
        // 和 ts 不一样，TS 相当于定义了四个值，但是 rust 是定义了四个类型
        Spring,
        Summer,
        Autumn,
        Winter,
    }

    // 创建了两个实例
    let mut summer1: Season = Season::Summer;
    let summer2: Season = Season::Summer;

    // oop 的语言有集成，不同的类型可以抽象出父类用于集成
    // rust 中没有没有 class，更没有集成，但是有枚举可以用于组合他们，便于在在定义函数参数可以传递所有子类

    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    // 标准库定义 IpAddr 的方式
    enum IpAddr {
        // 每个成员的关联数据都是不同的结构体
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // 这个例子演示了定义有关联数据的枚举成员和定义结构体很像，处理没有 struct 关键字
    enum Message {
        Quit,                       // 就像定义了单元结构体 struct Quit;
        Move { x: i32, y: i32 },    // struct Move { x: i32, y: i32 };
        Write(String),              // struct Write(String);
        ChangeColor(i32, i32, i32), // struct ChangeColor(i32, i32, i32
    }

    // 枚举允许定义方法
    impl Message {
        fn call(&self) {}
    }

    let m = Message::Quit;
    m.call();
    // 结构体用于构建复合类型，枚举可以用于组合不同结构体

    // ------------------------ Option -----------------------------
    // 在 java 中，任何一个变量都是可以是 null，这就导致编译器无法检测出一个变量是 null 还是具体的类型值
    /*
    // 下面的 代码在 java 中时可以编译器通过的
    public class Test {
        private int a=1;
        private int b=2;
        public static void main(String[] args) {
            // TODO Auto-generated method stub
            Test t1 = new Test();
            Test t2 = null;
            System.out.println(t1.a);
            System.out.println(t2.a);
            System.out.println(t2.c());
        }
        public String c(){
            return "123";
        }
    }
     */

    // 在 TS 中可以通过选项将 strictNullChecks 识别为一个独立的类型
    // 意味着 const num: number = null; 会编译报错
    // 这种方式算是编译器处理空异常的一种方案吧

    // 和 ts 不一样的是，rust 引入 Option 类型来在编译期间检查来处理空异常问题

    let none: Option<i32> = None;
    let num = Some(666);

    // Option<T> 的实例就像 java 中一个可能为 null 的 T 类型实例，javac 认为他俩是同类型
    // 但是 rustc 肯定是将 Options<T> 和 T 视为不同的类型
    // 正是因为 Option<T> 和 T 是不同的类型，rust 能检测出那些把 Option<T> 当 T 用的不安全的情况
    // 当需要表达一个类型 T 可能未空时我们需要使用 Option<T>，就像 TS 中 T | null
    // 那么如何处理 Option<T> 有值和无值的情况？ 在 TS 是直接判断对象是否为 null
    // rust 是使用 match 表达式

    let c = 'a';
    println!("{}", std::mem::size_of_val(&c));

    // ------------------------ match 表达式 -----------------------------

    // match  表达式初体验
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        let c = Coin::Penny;
    }

    {
        #[derive(Debug)] // 这样可以立刻看到州的名称
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    // ------------------------ if let -----------------------------
    let config_max: Option<i32> = None;
    // 这里没写 else，丢失了模式匹配的穷举特性
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    fn test_if_left() -> Option<i32> {
        let v: Option<i32> = None;

        if let Some(i) = v {
            v
        } else {
            return None
        }
    }
}

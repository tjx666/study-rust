#![allow(dead_code)]
#![allow(unused_variables)]

// fn basic_example() {
//     let a;

//     {
//         let b = 6;
//         // `b` does not live long enough borrowed value does not live long enough
//         a = &b;
//     }

//     println!("{}", a);
// }

// missing lifetime specifier
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `x` or `y`
// 此时有两个输入引用，一个输出引用
// 因为 rust 要保证引用的始终有效性，但是借用检查器无法自动推导出 str 的引用和 x, y 的关系
// 如果是一个参数，rust 默认会认为和输入引用一致
// 但是两个，返回的引用是和 x 一致还是和 y 一致

// 我们声明函数的时候有时候可以省略返回值，那是因为大多数情况 rust 可以推导出返回值的类型
// 但是例如你声明了一个 trait，可能返回两种不同的类型，那么 rust 就无法确定返回的具体类型
// 生命周期是同理的
// 函数抽象了运算过程计算，需要声明参数和返回值类型，参数是变量
// 类型泛型抽象了类型之间关系，类型是变量
// 生命周期泛型抽象了输入生命周期和返回生命周期之间的关系，生命周期是变量
// 因为函数是被调用是被调用的，我们需要在编译期间确定好它的输入输出，这包括参数的个数，参数的类型，参数和返回值之间的关系（类型和生命周期）
// 在借用检查器无法为我们做自动推导的时候我们就要手动标注生命周期关系
// fn longer(x: &str, y: &str) -> &str {
//     // rust 中没有条件运算符
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 生命周期也使用泛型的形式来表示
// 也就是说泛型既可以用于对类型的抽象，也可以用于对生命周期的抽象
fn longer1<'a>(x: &'a str, y: &str) -> &'a str {
    // rust 中没有条件运算符
    x
}

fn longer2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期约束取决于你函数的功能
fn just_return<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 这里的生命周期关系就是：返回的实例生命周期必须比 part 引用活的短
// 确保使用实例的时候，part 不是悬垂应用
struct ImportantExcerpt<'a> {
    // 之前的结构体都没包含引用
    // 因为你没有显示标注出字段的引用和结构体的实例生命周期的关系
    part: &'a str,
}

// 函数生命周期的省略规则
// 规则一：对于没有声明生命周期的函数，默认给每个参数声明赋予生命周期
// 等同于：fn test1<'a>(a: &'a str) {}
fn test1(a: &str) {}

// 规则二：如果只有一个引用参数，将这个引用的生命周期赋予返回值
// 等同于：fn test2<'a>(a: &'a str) -> &'a str {}
// fn test2(a: &str) -> &str {}

// 规则三：方法参数有 self 引用时，返回的引用生命周期和 self 一致
impl<'a> ImportantExcerpt<'a> {
    // 多个参数不符合第二条
    // 符合第三条，所以返回的字符串切片生命周期必须短于等于 self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn announce_and_return_part1<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期：'static
fn ss(a: &str, b: &str) -> &'static str {
    "aaa"
}

fn main() {
    /*
    生命周期在我学过的语言中都没有类似的概念，c++ 貌似有，c/java/python/js 我没接触过
    所以是个比较陌生的术语，但是它所表达的是一个很基础，通用的概念
    生命周期：引用的有效范围
    关键词分析：
    引用：
        - 我们在谈论生命周期的时候大都是在讨论引用，
        - 借用和引用规则又限制了我们使用引用时必须确保引用始终有效
        - 所有权机制中 owner 在离开作用域时会自动销毁，就可能出现引用无效
        - 那么就必须有套机制来确保引用始终有效，答案就是：rust 编译器的借用检查器和生命周期泛型
    有效：
        - 其实就是引用声明开始到当前作用于结束

    生命周期的作用：就是确保在使用引用的时候引用有效，也就是避免出现悬垂引用，也就是空指针/野指针
     */

    let a = &"a".to_string();
    {
        let b = &"b".to_string();
        longer2(a, b);
    }

    let s = "some important exception";
    let e = ImportantExcerpt { part: s };
}

/*
总结：
泛型让我们可以编写适用于不同类型但逻辑类似的代码
trait 让我们可以抽象出类型共享的行为，也可以让我们函数参数接受不同的类型
trait bounds 让我们使用泛型抽象多种类型共同的逻辑的同时还可以使用特定的行为
lifetime 让我们可以约束函数的返回值和参数，以及结构体和引用字段的生命周期关系，避免出现悬垂引用
而这一切的检查工作都是编译器的，所以知道了为啥 rust 编译慢了吧，因为要干的活太多！也是为啥很多程序员说 rust 编译器很烦！
 */

/*
结构体

结构体的作用
1. 关联一组数据（当然元组也可以）
2. 你可以给复合类型命名有意义的字段，并且不要求顺序，通过字段访问也远比下标访问更易用
 */

#[allow(unused)]

fn main() {
    // ------------------------ 定义结构体 -----------------------------
    // 注意是使用逗号分隔字段，习惯了在 ts 里面用分号分隔接口字段，切换到 rust 到容易敲错
    // struct 关键字 + 结构体名
    struct Layer {
        // left 是字段名
        left: i32,
        // rust 不允许声明某个字段的可变性
        top: i32,
        width: i32,
        // 最后的分号可加可不加
        height: i32,
    }

    // ------------------------ 实例化结构体 -----------------------------
    // 需要初始化所有字段
    // 使用 mut 声明可变的实例
    let mut layer = Layer {
        // 初始化时，字段的顺序可以不按照声明结构体的顺序
        top: 0,
        left: 0,
        height: 150,
        width: 300,
    };

    // missing fields `height`, `top` and `width` in initializer of `Layer`
    // missing `height`, `top` and `width`
    // let layer1 = Layer {
    //     left: 0
    // };

    println!("{}", layer.left);
    // 只要 layer 是可变的就可以修改 layer 的字段
    layer.top = 10;

    fn init_layer() -> Layer {
        // 结构体初始化时表达式
        // 是表达式就可以作为函数返回值
        Layer {
            top: 10,
            left: 10,
            width: 110,
            height: 110,
        }
    }

    fn build_layer(top: i32, left: i32) -> Layer {
        Layer {
            // 字段初始化简写
            top,
            left,
            width: 300,
            height: 150,
        }
    }

    struct User {
        active: bool,
        // 这里使用 String 而不是 str&，因为这样保证了结构体拥有所以数据，结构体的引用有效，结构体成员也是有效的
        // 如果用 str&，需要设置生命周期标识符
        username: String,
        email: String,
        sign_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("ytj"),
        email: String::from("ytj2713151713@gmail.com"),
        sign_count: 0,
    };

    // 结构体更新语法
    let user2 = User {
        // 必须写在最后
        // ..user1,
        // active 和 username 顺序不重要
        active: false,
        username: String::from("changfeng"),
        // 使用两个点加另一个实例变量，类似 JS 解构赋值，但是解构赋值是 3 个点
        ..user1
    };

    // let user3 = User {
    //     // 必须放最后也限制了不能同时解构两个
    //     ..user1,
    //     ..user2,
    // };

    // 此时 user1 失效了, 因为 user1 的 email 所有权已经在声明 user2 的时候被转移了
    // 部分成员所有权失效直接导致解构体实例本身所有权失效
    // println!("{}", user1.email);

    // ------------------------ 元组结构体 -----------------------------
    // 元组也可以关联一组数据，但是它没用类型名称，你无法给一个元组取一个有意义的名称
    let point = (0, 0);
    // 声明元组结构体和普通结构体的区别在于把花括号换成了圆括号
    struct Point(i32, i32);
    let point1 = Point(100, 100);
    // 简言之，元组结构体是披着结构体语法外衣的元组
    // 语义和元组是一样的，只不过用了结构体的语法来声明和定义
    println!("{}", point1.0);

    // rust 不是结构化类型，因此不存在两个元组成员类型和顺序相同就可以相互赋值
    struct Color(i32, i32);
    // expected struct `Point`, found struct `Color`
    // point1 = Color(100, 100);

    // ------------------------ 类单元结构体 -----------------------------
    // 类似于单元类型 (), 类单元结构体表示啥字段也没有逇结构体
    // 定义时不需要声明字段
    struct AlwaysEqual;
    let always_equal = AlwaysEqual;

    // ------------------------ 输出结构体 -----------------------------
    // 方法 1：
    // 使用 derive attribute 去给我们的自定义类型添加其它 trait 的行为
    // 就可以在 println1 宏里面使用针对结构体的输出格式
    // Debug 是派生 trait
    #[derive(Debug)]
    struct Line {
        start: (i32, i32),
        end: (i32, i32),
    }

    let line = Line {
        start: (0, 0),
        end: (0, 10),
    };
    println!("{:?}", line);
    /* =>
    Line { start: (0, 0), end: (0, 10) }
     */
    println!("{:#?}", line);
    /* =>
    Line {
    start: (
        0,
        0,
    ),
    end: (
        0,
        10,
    ),
     */

    // 方法二：派生 Debug trait, 使用 dbg! 宏
    // 这个宏会将输出输出到 stderr, 并附带使用处的代码行号
    dbg!(line);
    /* =>
    [src/main.rs:158] line = Line {
        start: (
            0,
            0,
        ),
        end: (
            0,
            10,
        ),
    }
     */

    // 看看源码实现
    /*
       $crate::eprintln!("[{}:{}] {} = {:#?}",
                   $crate::file!(), $crate::line!(), $crate::stringify!($val), &tmp);
    */
    // !: 需要注意的是 dbg! 宏会转移所有权，但同时也会返回所有权
    // 由于上面没有对 line 重新赋值，因此，此时 line 已经失效了
    // println!("{:?}", line);

    // ------------------------ 定义 方法 -----------------------------
    // 和很多其它语言一样，方法可以理解非全局的函数，或者说成员方法叫函数
    // 聚体到 rust，在结构体，枚举，trait 对象内的函数叫方法
    // rust 相比较特殊的是，它的第一个参数始终是 self
    struct Rectangle {
        width: i32,
        height: i32,
    }

    // 使用 impl 块来声明和 Rectangle 相关联的东西
    impl Rectangle {
        // 方法的第一个参数必定是 self
        // 当然形式有很多：self, &self, mut self, &mut self
        // &self 是 self: &Self 的语法糖
        // 这里不使用 self，是因为不需要所有权
        fn area(&self) -> i32 {
            self.width * self.height
        }

        // 如果调用方法后需要原本实例失效比较适合使用 self 声明
        // 例如将实例转换成另外一种实例的时候
        fn transform_to_string(self) -> String {
            let mut s = String::from(self.width.to_string());
            s.push_str(", ");
            s.push_str(&self.height.to_string());
            s
        }
    }

    let rect = Rectangle {
        width: 100,
        height: 100,
    };
    rect.area();
    // 其实等同于 ，rust 自动引用了
    (&rect).area();

    // 关联函数：impl 块里的函数都是关联函数，声明了 self 参数的函数称之为方法
    // 类似于其它语言中的静态方法，如果关联函数可以不声明 self 参数
    // 同时这里再一次多 Rectangle 使用了 impl, 说明 rust 允许对一个结构体多次 impl
    impl Rectangle {
        fn square(size: i32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
}

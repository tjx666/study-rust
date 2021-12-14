/*
比起 npm，cargo 更像 cmake, gradle
cargo 职责是管理包的 构建 和 依赖
npm 更多的是管理依赖，构建啥的是用户自己配置

使用 cargo 时，需要将当前工作路径切到 Cargo.toml 所在路径，也就是包的根路径
rust 开发者称为：rustaceans
rust 包称为 crate，意思是木箱

常用的 cargo 命令:

`cargo new project-name` 新建一个 rust 包
`cargo init` 在已存在的文件夹初始化 rust 包
`cargo check` 不生产可执行文件，只检查是否通过编译
`cargo build` 开发编译，没有优化，生成可执行文件 + debug 信息，可执行文件在 target/debug/xxx
`cargo build --release`  生产编译，会对生成的代码进行优化，可执行文件在 target/release/xxx

使用 `cargo` 编译时，如果代码内容没有变化，不会重新编译
实测这个变化不是语义变化，只要文件内容变化就会重新编译，例如加个空行也会重新编译

开发编译和生产编译对比：
1. 产物，开发编译会额外附加调试信息，代码未经优化，体积会更大
2. 编译时间，由于生产编译需要优化代码，编译时间会更长
3. 运行速度，生产环境可执行文件运行速度更快，因此如果要做 benchmark，一般都是需要用 release 版
 */

fn main() {
    println!("Hello, cargo!");
}

## 包和 crate
#### 定义模块
    通过 mod 关键字来定义一个模块，模块可以嵌套，模块之间的关系有父子、兄弟，src 目录下的 lib.rs 和 main.rs 文件在 crate 模块结构的根组成了 crate 模块
#### 模块调用
    使用绝对路径还是相对路径访问模块取决于对未来的预期，若未来可能同时移动模块定义代码和调用代码，则使用相对路径较好；若未来更可能单独移动其中之一，则使用绝对路径方法更好
    - 绝对路径
        crate 或 crate 字面值后跟 ::
    - 相对路径
        self、super、当前模块标识符后跟 ::
#### pub 暴露路径
    在一个模块中调用另一个模块的子模块或函数默认是不允许的，需要使用 pub 关键字表示公有状态
    一个模块的结构体和结构体字段如果想被另一个模块使用，也需要使用 pub
    pub enum 之后直接就全部公有化
### 使用 use 引入作用域
    这种方式简洁方便，是很常用的模块引入方法
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {

            }
        }
    }
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
    以上代码有两个问题：
    1. 为什么不是直接使用 use 引入到最后的 add_to_waitlist 函数，而是引入的 hosting 模块？
    因为如果直接 use::front_of_houst::hosting::add_to_waitlist 固然是可以的，但是就无法知道 add_to_waitlist 函数是在哪里定义的，而像我们上面的抒写方法则可以知道函数是在哪个模块被定义的，考虑以下如果有两个同名函数所属不同模块应该就可以明白了，当然也可以通过 as 关键字解决
    use std::fmt::Result;
    use std::io::Result as IoResult;
    fn function1() -> Result {
    // --snip--
    }
    fn function2() -> IoResult<()> {
    // --snip--
    }
    2. 如果使用 use 引入作用域之后其他作用域想要使用这个作用域引入的模块怎么办？
    使用 pub use '重导出'
    mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    }
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
### 使用外部包
    1. 在 Cargo.toml [dependencies] 下加入一行以下形式代码 [name] = [version]，例：
    Cargo.toml 文件
    [dependencies]
    rand = "0.5.5"
    2. 使用 use 引入作用域
    use rand::Rng;
    需要注意的是 std 标准库的使用不需要在 Cargo.toml 引入，直接 use 即可
    use std::collections::HashMap;
### 嵌套路径
    嵌套的路径可以在一行中将多个带有相同前缀的项引入作用域，例：
    use std::io::{self, Write}; 
    等价于
    use std::io;
    use std::io::Write;
### glob 运算符
    glob 可以将所有公有定义引入作用域，常用于测试，例：
    use std::collections::*;
### 模块分割为不同文件
    使用 mod 最后使用分号的形式告诉编译器寻找另一个文件名与此 mod 名称相同的文件，例：
    lib.rs 文件
    mod front_of_house;
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    front_of_house.rs 文件
    pub mod hosting() {
        pub fn add_to_waitlist() {}
    }

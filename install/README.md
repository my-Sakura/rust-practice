## install
    运行以下命令安装 rust SDK
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

## rustc
    rustc 是 rust 编译工具
    rustc [文件名] ------ 命令会在当前目录生成编译后的二进制文件

## cargo
    cargo 是 rust 包管理工具
    cargo new [项目名称] ------ 创建项目于当前目录，项目内容包括 src 目录、Cargo.toml 项目配置文件、.git 文件和 .gitignore 文件
    cargo build ------ 命令将编译当前项目生成二进制文件于 target/debug/ 目录
    cargo run ------ 将编译和运行合为一步
    cargo check ------ 检查代码确保代码能够编译成功，但不生成二进制文件(推荐在编译前使用)


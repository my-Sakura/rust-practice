## 结构体
### 定义结构体
    使用 struct 关键字
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

### 创建实例
    let user1 = User {
        username: String::from("admin"),
        email: String::from("admin@111.com"),
        active: true,
        sign_in_count: 1,
    };

### 访问和更改结构体字段
    使用 (.) 后面跟一个字段名的方式访问字段
    let mut user2 = User {
    username: String::from("user"),
    email: String::from("user@111.com"),
    active: true,
    sign_in_count: 2,
    };
    user2.active = false;

### 字段初始化简写语法(field init shorthand)
    如果函数参数名和结构体字段名完全相同，可使用字段初始化简写语法构造结构体实例
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

### 结构体更新语法(struct update syntax)
    使用旧结构体实例来初始化新结构体实例
    let user3 = User {
        email: String::from("sakura@111.com),
        ..user1
    };

### 元组结构体
    使用 struct 关键字
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

### 方法
    使用 impl 关键字给结构体绑定一个方法，方法的第一个参数可以为 self、&self、&mut self

### 关联函数
    关联函数不需要第一个参数为 self，使用 :: 调用，一般用作构造函数

### 打印结构体
    1. 结构体上 #[drive(Debug)]
    2. 占位符为 {:?} 或 {:#?}
    #[drive(Debug)]
    struct User {
        name: String,
    }
    let user1 = User {
        name: String::from("John),
    };
    println!("{:#?}", user1)

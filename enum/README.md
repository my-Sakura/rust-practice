## 枚举
### 定义
    使用关键字 enum 定义一个枚举
    enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    }

### 访问字段
    使用 :: 访问枚举字段
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

### 方法
    和结构体一样使用 impl

### Option
    Option 是标准库定义的枚举，存在于 preclude 中，不需要显示引入即可以使用，定义如下:
    enum Option<T> {
        Some<T>,
        None,
    }
    let some_string = Some("a string");
    let n: Option<i32> = None;
    注意： 使用 None 需要指定 T

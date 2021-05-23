## match
### QuickStart
    fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

### if let 语法糖
    当只想 match 一个分支时，可以使用 if let，当然也可以使用 if let else 直接替换 match
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("3");
    }

## 总结
    1. match 后面可以是任意类型，但常用语于枚举结合
    2. => 前部分为匹配条件
    3. => 为对应运行代码
    4. match 是穷尽的，需要匹配所有情况
    5. 可以使用 _ 表示剩下所有情况
    6. 可以使用 if let else 替换 match
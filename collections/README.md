## 常见集合
### vector
#### 定义
    标准库提供了 vec! 宏，vector 是用泛型实现的
    let vec: <i32> = Vec::new();
    或
    let vec = vec![1, 2, 3];
#### 更新值
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
#### 删除
    let v = vec![1, 2, 3];
    v.pop();
    v.pop();
#### 访问值
    使用索引或者 get 方法，两者区别在于访问索引外元素时，索引方法会 panic，get 方法只会返回 None，而不会 panic
    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    match v.get(2) {
        Some[third] => println!("The third element is: {}", third),
        None => println!("There is no third element"),
    }
#### 遍历 vector 
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
#### 使用 enum 存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blud")),
    ];

### 字符串
    字符串分为 &str 和标准库 String，两者都是 UTF-8 编码格式
#### 新建字符串
    to_string() 和 String::from() 效果一样
    let s1 = "hello";
    let s2 = s1.to_string();

    let s3 = "hello".tostring();

    let s4 = String::from("hello");
#### 更新字符串
    1. 使用 push_str() 拼接字符串
    let mut s = String::from("hello");
    s.push_str(".world");
    2. 使用 push() 拼接字符
    let mut s = String::from("lo");
    s.push('l');
    3. 使用 +
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    注意：s1 + s3 获取了 s1 所有权，故原 s1 失效
    4. 使用 format 宏
    let s1 = "a";
    let s2 = "b";
    let s3 = "c";
    let join = format!("{}{}{}", s1, s2, s3);
#### 字符串 slice
    使用 [] 和 range
    let hello = "Здравствуйте"
    let s = &hello[0..4];
    因为这里每个字母两字节，所以是前八个字母
    注意: &hello[0..1] 会出错
#### 遍历字符串
    for c in "abc".chars() {
        println!("{}", c);
    }
### 哈希 map
#### 创建且插入 map
    use std::collections::HashMap;
    1. 使用标准库 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 12);
    2. 使用 vector collect 方法
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 12];
    let scores HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collecc();
    3. 条件插入
    使用 entry + or_insert 方法，如果没有 Blue 则插入 10，返回 10 的可变引用；否则，返回已有 value 的可变引用
    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Yellow")).or_insert(12);

#### 所有权
    使用 insert() 后，原变量所有权转移给 map
#### 通过 key 获取 value
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 12);
    1. 通过 get
    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    2. 通过遍历
    for (key, value) in &scores {
        println!("key:{}, value:{}", key, value);
    }




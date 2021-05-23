## 所有权
### 堆、栈
    栈和堆都是代码在运行时可供使用的内存
#### 栈
    先入后出；编译时内存大小已知且固定，存储数据直接入栈顶
#### 堆
    编译时内存大小未知且有可能变化；将数据存储在堆上比存储在栈上慢，因为需要操作系统在堆上寻找一块足够大的内存进行分配；
    存储也比栈慢，因为需要通过访问指针的方式寻找数据

### 引用和借用
    1. & 符号代表引用，分为引用变量和引用类型
    2. 获取引用作为函数参数称为借用
    3. 默认引用不可修改
    fn main() {
        let s = String::from("hello);

        change(&s);
    }
    fn change(some_string: &String) {
        some_string.push_str(", world);
    }
    这段代码中 &s 和 &String 分别是引用变量和引用类型，将 &s 当作参数传入 change 函数称为借用， 
    同时这段代码是错的，因为默认引用不可修改
    那么如何通过借用修改原数据呢?
    只需要使用可变引用即可
    fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("s: {}", s);
    }

    fn change(some_string: &mut String) {
    some_string.push_str(", world");
    }
## 注意
    1. 不能再引用之后释放内存，如:
    fn dangle() -> &String { 
    let s = String::from("hello"); 
    &s 
    }
    2. 不能在同一作用域多次引用同一可变变量，如:
    let mut s = String::from("hello);
    let a = &mut s;
    let b = &mut s;


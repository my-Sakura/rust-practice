# 控制流
## if 条件语句
#### QuickStart
    let num = 3;
    if num < 2 {
        println!("smaller than 2");
    } else if num < 4 {
        println!("smaller than 4);
    } else {
        println!("value: {}", num)
    }
#### let 语句中使用 if
    因为 if 是一个表达式，所以可以在 let 右边使用它
    let conditon = true;
    let number = if conditon { 
        1
    } else {
        2
    };
    为什么最后要加分号？
    因为 let 整体是一个语句

## 循环语句
### loop
    loop 是无限循环的关键字，可以使用 break 关键字跳出循环
    ```rust
    let mut counter = 0;
    let result = loop { 
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    ```
### while


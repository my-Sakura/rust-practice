## slice
    和 golang 语法表示差不多
    let mut s = String::from("hello");
    let all = &s[..];
    let front = &s[..2];
    let back = &s[3..];
    let mid = &s[1..3];

fn main() {
    let string = "hello world";
    println!("{}", first_word(&string));
}

// first_word get the s first word
fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
        return &s[..i];
        }
    }

    &s[..]
}
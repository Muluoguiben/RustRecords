fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s[..]);
		println!("the first word is: {}", word);
		
		let my_string_literal = "hello world";
    // first_word 中传入字符串字面值的 slice
    let word2 = first_word(&my_string_literal[..]);
		println!("the second word is: {}", word2);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
		let word3 = first_word(my_string_literal);
		println!("the third word is: {}", word3);
		
    // // ==
    // let slice = &s[0..2];
    // let slice = &s[..2];
    // let len = s.len();
    // // ==
    // let slice = &s[3..len];
    // let slice = &s[3..];
    // // ==
    // let slice = &s[0..len];
    // let slice = &s[..];
}

fn first_word(s: &str ) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

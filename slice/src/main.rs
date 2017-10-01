fn main() {
    let my_string = String::from("hello world");

    let word1 = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word2 = first_word(my_string_literal);

    println!("word1: {} | word2: {}", word1, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

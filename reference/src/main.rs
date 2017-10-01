fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = no_dandle();

    println!("{}", s)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn no_dandle() -> String {
    let s = String::from("Hello");
    s
}

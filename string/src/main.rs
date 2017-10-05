fn main() {
    {
        let s1 = String::from("hello");
        let s2 = "hello".to_string();
    }
    {
        let mut s1 = "foo".to_string();
        let s2 = s1.push_str("bar");
    }
    {
        let mut s = "lo".to_string();
        s.push('l');
    }
    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        let s4 = "hello, ".to_string() + "world";
    }
    println!("Hello, world!");
}

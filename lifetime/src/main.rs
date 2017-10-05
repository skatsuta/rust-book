use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    {
        let s1 = "abcd".to_string();
        let s2 = "xyz";

        let res = longest(s1.as_str(), s2);
        println!("The longest string is {}", res);
    }
    {
        let s1 = "long string is long".to_string();
        {
            let s2 = "xyz".to_string();
            let result = longest(s1.as_str(), s2.as_str());
            println!("The longest string is {}", result);
        }
    }

    {
        let novel = "Call me Ishmael. Some years ago...";
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }
}

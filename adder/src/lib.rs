pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn index_out_of_bounds() {
        vec![1][5];
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // take an hour to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: "sneaker".to_string(),
        },
        Shoe {
            size: 13,
            style: "sandal".to_string(),
        },
        Shoe {
            size: 10,
            style: "boot".to_string(),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size
            .iter()
            .map(|s| &s.style)
            .collect::<Vec<&String>>(),
        vec!["sneaker", "boot"]
    );
}

#[test]
fn use_other_iter_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18);
}

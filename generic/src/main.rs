fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers).unwrap();
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars).unwrap();
    println!("The largest char is {}", result);

    let both_int = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 5, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

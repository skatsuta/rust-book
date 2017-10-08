struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

fn main() {
    {
        let mut stack = vec![1, 2, 3];

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    {
        let v = vec![1, 2, 3];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }
    {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("O neither axis: ({}, {})", x, y),
        }
    }
    {
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        assert_eq!(135, sum_of_squares);
    }
    {
        let origin = Point { x: 0, y: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => println!("first = {}, last = {}", first, last),
        }
    }
    {
        let name = Some(String::from("Bors"));

        match name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }

        println!("name is: {:?}", name);
    }
    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }
    {
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id @ 3...7 } => println!("Found an id in range: {}", id),
            Message::Hello { id: 10...12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

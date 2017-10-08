use std::ops::Add;
use std::fmt;
use std::io;
use std::io::{BufWriter, Write};

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

struct Ref<'a, T: 'a>(&'a T);

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

trait Foo {}

struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> {}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, Point { x, y }: Point) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(PartialEq, Debug)]
struct Millimeters(u32);

#[derive(PartialEq, Debug)]
struct Meters(u32);

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        let mut buf = io::BufWriter::new(io::stdout());
        writeln!(buf, "{}", "*".repeat(len + 4));
        writeln!(buf, "*{}*", " ".repeat(len + 2));
        writeln!(buf, "* {} *", output);
        writeln!(buf, "*{}*", " ".repeat(len + 2));
        writeln!(buf, "{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    {
        let num = 5;
        let obj = Box::new(Bar { x: &num }) as Box<Foo>;
    }
    {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    {
        let l1 = Millimeters(10);
        let l2 = Millimeters(20);
        let l3 = Meters(1);
        assert_eq!(Millimeters(30), Millimeters::add(l1, l2));

        let l1 = Millimeters(10);
        assert_eq!(Millimeters(1010), Millimeters::add(l1, l3));
    }
    {
        let p = Point { x: 1, y: 2 };
        p.outline_print();
    }
    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

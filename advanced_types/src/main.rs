fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }
    {
        type Thunk = Box<FnOnce() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {}
    }
    {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }
    {
        let nums = vec![1, 2, 3];
        let strs: Vec<_> = nums.iter().map(|i| i.to_string()).collect();
    }
    {
        let nums = vec![1, 2, 3];
        let strs: Vec<_> = nums.iter().map(ToString::to_string).collect();
    }
}

fn return_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

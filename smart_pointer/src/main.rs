#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

use std::ops::Deref;

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    vec![]
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn main() {
    {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);
    }
    {
        let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
        assert_eq!(6, x);
    }
    {
        let fav_song = Mp3 {
            audio: vec![1, 2, 3],
            artist: Some("Nirvana".to_string()),
            title: Some("Smells Like Teen Spirit".to_string()),
        };
        assert_eq!(vec![1, 2, 3], *fav_song);

        compress_mp3(&fav_song);
    }
    {
        let c = CustomSmartPointer { data: "some data".to_string() };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("Wait for it...");
    }
}

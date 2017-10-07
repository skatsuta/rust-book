#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    Cons(Rc<RefCell<i32>>, Rc<RcList>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;

fn immut_borrows(a: &String) {
    println!("a is {}", a);
}

fn mut_borrows(b: &mut String) {
    *b += "bar"
}

fn demo(r: &RefCell<String>) {
    immut_borrows(&r.borrow());
    mut_borrows(&mut r.borrow_mut());
    immut_borrows(&r.borrow());
}

fn main() {
    {
        let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
        println!("rc = {}", Rc::strong_count(&a));
        let b = List::Cons(3, Rc::clone(&a));
        println!("rc after creating b = {}", Rc::strong_count(&a));
        {
            let c = List::Cons(4, Rc::clone(&a));
            println!("rc after creating c = {}", Rc::strong_count(&a));
        }
        println!("rc after c goes out of scope = {}", Rc::strong_count(&a));
    }
    {
        let data = RefCell::new("foo".to_string());
        demo(&data);
    }
    {
        let val = Rc::new(RefCell::new(5));

        let a = RcList::Cons(Rc::clone(&val), Rc::new(RcList::Nil));
        let shared_list = Rc::new(a);

        let b = RcList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&shared_list));
        let c = RcList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&shared_list));

        *val.borrow_mut() += 10;

        println!("shared_list after = {:?}", shared_list);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

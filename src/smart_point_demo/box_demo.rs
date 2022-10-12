use self::List::{ Cons, Nil };

pub fn create_box() {
    let b = Box::new(5);

    println!("box value is: {}", b);
}

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil
}

pub fn create_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list is: {:?}", list);
}
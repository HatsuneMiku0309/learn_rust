use std::ops::Deref;

pub fn normal_deref() {
    let x = 5;
    let y = &x;

    println!("y is: {}", *y);
}

pub fn box_deref() {
    let x = 5;
    let y = Box::new(x);

    println!("y is: {}", *y);
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn mybox_deref_fail() {
    let x = 5;
    let _y = MyBox::new(x);

    // println!("y is: {}", *y); // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
}



impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn mybox_deref() {
    let x = 5;
    let y = MyBox::new(x);

    println!("y is: {}", *y);
}

fn hello(name: &str) {
    println!("Hello, {}1", name);
}

/**
 * Rust does deref coercion when it finds types and trait implementations in three cases:
 *  * From &T to &U when T: Deref<Target=U>
 *  * From &mut T to &mut U when T: DerefMut<Target=U>
 *  * From &mut T to &U when T: Deref<Target=U>
 */
pub fn mybox_deref_implicit_deref_coercions_with_functions_and_methods() {
    let m = MyBox::new(String::from("Rust"));

    hello(&m);
    // hello(&(*m)[..]);
}
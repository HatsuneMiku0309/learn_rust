use std::{ops::Add};

pub fn create_string_demo() {
    let mut a = String::new();
    a.push_str("String::new push_str");

    let mut s = String::from("Hello world");
    s.push_str(" !!");

    let data = "init data";
    let ss = data.to_string();

    println!("[{FILE_NAME}][13] :: {}", a);
    println!("[{FILE_NAME}][14] :: {}", s);
    println!("[{FILE_NAME}][15] :: {}", ss);
}

pub fn add_string_oper() {
    let s1 = String::from("Hello");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;

    println!("[{FILE_NAME}][23] :: add oper value: {}", s3);
}

pub fn add_string_fun() {
    let s1 = String::from("Hello");
    let s2 = String::from(" world!");
    /*
        fn add(self, s &str) -> String

        why can run when s is `&String` type?
        The reason we’re able to use &s2 in the call to add is that the compiler can `coerce` the &String argument into a &str.
        When we call the `add` method, `Rust` uses a `deref coercion`, which here turns &s2 into &s2[..]
     */
    let s3 = s1.add(&s2);

    println!("[{FILE_NAME}][38] :: add fun value: {}", s3);
}

pub fn add_string_multi_oper() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("[{FILE_NAME}][47] :: add multi oper value: {}", s);
}

pub fn add_string_multi_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("[{FILE_NAME}][56] :: add format! fun value: {}", s);
}

pub fn indexing_into_string() {
    let s1 = String::from("hello");
    /*
        error, 因為不同文字所佔用的bytes不同。
        ex. 
            * Hola: len = 4
                H: 48 (1 bytes)
            * Здравствуйте: len = 24 (不是12, 因為一個char佔用2 bytes)
                З: 208 151 (2 bytes)
     */ 
    
    // let h = s1[0];
}

static FILE_NAME: &str = "collections_demo/string_demo.rs";

#[allow(dead_code)]
mod stdin_demo;
#[allow(dead_code)]
mod guess_demo;
#[allow(dead_code)]
mod first_word;

#[allow(dead_code)]
mod match_demo;

// use match_demo::*;
// use match_demo::enum_demo;
// use match_demo::match_demos;
// use match_demo::if_let;
#[allow(dead_code)]
#[allow(unused_imports)]
use crate::match_demo::{
    enum_demo,
    match_demos,
    if_let1 as if_let
};

#[allow(dead_code)]
mod mysql_demo;
#[allow(dead_code)]
#[allow(unused_imports)]
use crate::mysql_demo::{
    conn_demo
};

#[allow(dead_code)]
mod collections_demo;
#[allow(dead_code)]
#[allow(unused_imports)]
use collections_demo:: {
    vec_demo,
    string_demo,
    hashmap_demo
};

#[allow(dead_code)]
mod error_demo;
#[allow(dead_code)]
#[allow(unused_imports)]
use error_demo:: {
    index as err_demo
};

fn main() {
    let heart_eyed_cat = '😻';
    println!("Run [{FILE_NAME}] say miao~~~ {} \n", heart_eyed_cat);

    // stdin_demo::stdin();
    // guess_demo::comparing_the_guess_to_the_secret_number();
    // {
    //     let s = String::from("hello world");
    //     let a = first_word::first_word(&s);
    //     let b = first_word::first_word_optimization(&s);
    //     // first_word::learn_demo();
    //     println!("[{FILE_NAME}][x] :: {a}, {b}");
    // }
    
    // {
    //     println!("=-=-=-=-=-=-=-=-=-=-=-=");
    //     enum_demo::enum_ipaddr_demo();
    //     println!("=-=-=-=-=-=-=-=-=-=-=-=");
    //     enum_demo::enum_option_demo();
    //     println!("=-=-=-=-=-=-=-=-=-=-=-=");
    //     enum_demo::enum_option1_demo();
    // }
    
    // {
    //     let coin = match_demos::Coin::Penny;
    //     let coin2 = match_demos::Coin::Nickel;
    //     let coin3 = match_demos::Coin::Quarter(match_demos::UsState::Taiwan);
    //     let coin4 = match_demos::Coin::Dime;
    //     let a = match_demos::value_in_cents(coin);
    //     let b = match_demos::value_in_cents(coin2);
    //     let c = match_demos::value_in_cents(coin3);
    //     let d = match_demos::value_in_cents(coin4);
    //     println!("[{FILE_NAME}][x] :: a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    //     println!("");
    //     let five = Some(5);
    //     let six = match_demos::match_option(five);
    //     let w = match_demos::match_option(None);
    //     println!("[{FILE_NAME}][x] :: six: {:?}, w: {:?}", six, w);
    // }

    // {
    //     if_let::if_let_demo(3);
    //     if_let::if_let_demo_simple(4);
    //     let mut input = match_demos::Coin::Quarter(match_demos::UsState::Taiwan);
    //     let mut input2 = match_demos::Coin::Penny;
    //     let result = if_let::if_let_demo_simple2(&mut input);
    //     println!("[{FILE_NAME}][x] :: input: {:?}, result: {}", input, result);
    //     let result2 = if_let::if_let_demo_simple2(&mut input2);
    //     println!("[{FILE_NAME}][x] :: input2: {:?}, result2: {}", input2, result2);
    // }

    // {
    //     let qaq = conn_demo::mysql_conn_demo();
    //     println!("{:#?}", qaq);
    // }

    // {
    //     vec_demo::iterating_over_the_values_in_vector();
    //     string_demo::create_string_demo();    
    //     string_demo::add_string_oper();
    //     string_demo::add_string_fun();
    //     string_demo::add_string_multi_oper();
    //     string_demo::add_string_multi_format();
    //     string_demo::indexing_into_string();
    //     string_demo::iterating_over_string();
    //     hashmap_demo::new_hashmap();
    //     hashmap_demo::vec_to_hashmap();
    //     hashmap_demo::get_hashmap();
    //     hashmap_demo::cover_hashmap_value();
    //     hashmap_demo::only_insert_value_if_key_no_value();
    //     hashmap_demo::update_old_value();
    // }

    {
        // err_demo::get_file_demo("hello.txt");
        // err_demo::get_file_match_on_diff_error("hello.txt");
        err_demo::get_file_use_unwrap_or_else("hello2.txt");
        err_demo::get_file_use_unwrap_or_expect("hello.txt");
    }
    
}

static FILE_NAME: &str = "main.rs";

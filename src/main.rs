#[allow(dead_code)]
mod stdin_demo;
#[allow(dead_code)]
mod guess_demo;
#[allow(dead_code)]
mod first_word;

#[allow(dead_code)]
mod match_demo;

use std::collections::HashMap;
use std::vec;

// use match_demo::*;
// use match_demo::enum_demo;
// use match_demo::match_demos;
// use match_demo::if_let;
#[allow(unused_imports)]
use crate::match_demo::{
    enum_demo,
    match_demos,
    if_let1 as if_let
};

#[allow(dead_code)]
mod mysql_demo;
#[allow(unused_imports)]
use crate::mysql_demo::{
    conn_demo
};

#[allow(dead_code)]
mod collections_demo;
#[allow(unused_imports)]
use collections_demo:: {
    vec_demo,
    string_demo,
    hashmap_demo
};

#[allow(dead_code)]
mod error_demo;
#[allow(unused_imports)]
use error_demo:: {
    index as err_demo
};

#[allow(dead_code)]
mod generic_data_demo;
#[allow(unused_imports)]
use generic_data_demo:: {
    generic_demo,
    trait_demo
};

#[allow(dead_code)]
mod closures_demo;
#[allow(unused_imports)]
use closures_demo:: {
    iterator,
    improving_io
};

#[allow(dead_code)]
mod smart_point_demo;
#[allow(unused_imports)]
use smart_point_demo:: {
    box_demo,
    deref_demo,
    drop_demo,
    rc_demo
};

#[allow(dead_code)]
mod good_test;
#[allow(unused_imports)]
use good_test:: {
    my_test
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

    // {
    //     // err_demo::get_file_demo("hello.txt");
    //     // err_demo::get_file_match_on_diff_error("hello.txt");
    //     err_demo::get_file_use_unwrap_or_else("hello2.txt");

    //     // this go fail. when no found file
    //     // err_demo::get_file_use_unwrap_or_expect("hello.txt");
    //     // let _s = match err_demo::shortcut_for_propagating_error("hello.txt") {
    //     //     Ok(data) => data,
    //     //     Err(e) => {
    //     //         println!("[{FILE_NAME}][132] :: error: {:?}", e);
    //     //         panic!("fail {}", e)
    //     //     }
    //     // };
    //     let _s = match err_demo::shortcut_for_propagating_error("hello2.txt") {
    //         Ok(file) => file,
    //         Err(e) => panic!("{}", e)
    //     };
    //     println!("{}", _s);

    //     err_demo::check_guess(20);
    // }

    // {
    //     // generic_demo::generic_point();
    //     // generic_demo:: mixup_demo();

    //     let tweet = trait_demo::Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("off course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     };

    //     println!("1 則新推文：{}", tweet.summaryize());

    //     let article = trait_demo::NewsArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL."
    //         ),
    //     };

    //     println!("有新文章發佈！{}", article.summaryize());

    //     trait_demo::notify(&article);
    //     trait_demo::notify(&tweet);
    //     trait_demo::generic_notify(&article);
    //     trait_demo::generic_notify(&tweet);

    //     // trait_demo::mul_generic_notify(&article); the struct is not (Summary + Display) trait, should error
    //     trait_demo::mul_generic_notify(&tweet);
    //     trait_demo::where_mul_generic_notify(&tweet);

    //     let tweet2 = trait_demo::returns_summarizable();
    //     println!("returns_summarizable: {}", tweet2.display());
    // }

    // {
    //     // iterator::use_closure_get();
    //     // iterator::calling_next_directly();
    //     // iterator::using_other_iterator_trait_methods();
    //     let config = improving_io::set_args_in_config();
    //     println!("config: {:?}", config);
    // }

    // {
    //     box_demo::create_box();
    //     box_demo::create_list();
    //     deref_demo::normal_deref();
    //     deref_demo::box_deref();
    //     deref_demo::mybox_deref();
    //     deref_demo::mybox_deref_implicit_deref_coercions_with_functions_and_methods();
    //     drop_demo::drop_demo();
        
    // }

    {
        // my_test::unwrap_options();
    }

    let a = vec![2, 7, 11, 15];
    let qaq = two_sum(a, 9);
    println!("qaq: {:?}", qaq);
}

static FILE_NAME: &str = "main.rs";

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i,&v) in nums.iter().enumerate() {
        let target_v = target - v;
        // println!("target_v {}, {}", target_v, m.contains_key(&target_v));
        if m.contains_key(&target_v) {
            return vec![m[&target_v], i as i32];
        }
        else {
            m.insert(v, i as i32);    
        }
        
    }
    return vec![0,0];
}

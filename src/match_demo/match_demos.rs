#[derive(Debug)]
#[allow(dead_code)]
pub enum UsState {
    Alabama,
    Alaska,
    Taiwan
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

pub fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            let nickel_coin = 5;
            println!("[{FILE_NAME}][23] :: lucky coin: {:?} = {}", coin, nickel_coin);
            nickel_coin
        },
        Coin::Quarter(state) => {
            println!("[{FILE_NAME}][27] :: This 25 dollar of state {:#?}", state);
            25
        },
        // Coin::Dime => 10, 模擬 other, 所以 comment
        other => {
            println!("[{FILE_NAME}][32] :: Coin is {:?}", other);
            0
        }
    }
}

pub fn match_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

static FILE_NAME: &str = "match_demo/match_demos.rs";

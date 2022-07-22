pub mod if_let1 {
    use crate::match_demos;

    pub fn if_let_demo(input: u8) {
        let config_max = Some(input);
        match config_max {
            Some(max) => println!("[6] :: 最大值被設為 {}", max),
            _ => (),
        }
    }

    pub fn if_let_demo_simple(input: u8) {
        let config_max = Some(input);
        if let Some(max) = config_max {
            println!("[14] :: 最大值被設為 {}", max);
        }
    }

    pub fn if_let_demo_simple2(coin: &mut match_demos::Coin) -> i32 {
        let mut count = 0;
        if let match_demos::Coin::Quarter(state) = coin {
            println!("[21] :: This 25 dollar of state {:?}", state);
        } else {
            count += 1;
        };

        count
    }
}

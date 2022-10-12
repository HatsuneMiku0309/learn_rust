#[allow(unused_imports)]
use std::{ fs::{File, self}, io::{ErrorKind, Read, Error}};

pub fn get_file_demo(file:& &str) {
    let _f = match File::open(file) {
        Ok(file) => {
            println!("[{FILE_NAME}][7] :: open file success");
            file
        },
        Err(error) => panic!("[{FILE_NAME}][10] :: open file fail: {:#?}", error)
    };
}

pub fn get_file_match_on_diff_error(file: &str) {
    let f = File::open(file);

    let _f = match f {
        Ok(file) => {
            println!("[{FILE_NAME}][19] :: open file success");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(fc) => {
                    println!("[{FILE_NAME}][25] :: create file success");
                    fc
                },
                Err(e) => panic!("[{FILE_NAME}][28] :: Problem creating the file: {:?}", e),
            },
            other_err => {
                panic!("[{FILE_NAME}][31] :: Problem opening the file: {:?}", other_err);
            }
        }
    };
}

pub fn get_file_use_unwrap_or_else(file: &str) {
    let _f = File::open(file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let fc = File::create(file).unwrap_or_else(|error| {
                panic!("[{FILE_NAME}][41] :: when create file fail: {:?}", error);
            });
            println!("[{FILE_NAME}][43] :: create file success: {}", &file);

            fc
        } else {
            panic!("[{FILE_NAME}][47] :: open file fail: {:?}", error);
        }
    });

    println!("[{FILE_NAME}][51] :: open file success: {}", &file);
}

pub fn get_file_use_unwrap_or_expect(file: &str) {
    /*
        Use `expect` in the same way as `unwrap`: to return the file handle or call the `panic!` macro.
        
        The error message used by `expect` in its call to `panic!` will be the parameter that we pass to `expect`,
        tather than the default `panic!` message that unwrap uses.
    */
    let _f = File::open(file).expect("open {file} fail");
    // let _f = File::open(file).unwrap();
    /*
        Because this error message starts with the text we specified, `open {file} fail`,
        it will be easier to find where in the code this error message is coming from.

        If we use `unwrap` in multiple places,
        it can take more time to figure out exactly which `unwrap` is causing the panic because all `unwrap` calls that panic **print the same message**.
    */

    println!("[{FILE_NAME}][71] :: open file success: {}", &file);
}

pub fn propagating_error(file: &str) -> Result<String, std::io::Error> {
    let f = File::open(file);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}


pub fn shortcut_for_propagating_error(file: &str) -> Result<String, Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // let _s = fs::read_to_string(file)?;

    Ok(s)
}


pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("猜測數字必須介於 1 到 100 之間，你輸入的是 {}。", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


pub fn check_guess(value: i32) {
    let guess = Guess::new(value);

    let result = guess.value();
    println!("value: {}", result);
}

static FILE_NAME: &str = "error_demo/index.rs";

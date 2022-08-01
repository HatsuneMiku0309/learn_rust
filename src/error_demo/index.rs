use std::{fs::File, io::ErrorKind};

pub fn get_file_demo(file:& &str) {
    let _f = match File::open(file) {
        Ok(file) => {
            println!("[{FILE_NAME}][6] :: open file success");
            file
        },
        Err(error) => panic!("[{FILE_NAME}][9] :: open file fail: {:#?}", error)
    };
}

pub fn get_file_match_on_diff_error(file: &str) {
    let f = File::open(file);

    let _f = match f {
        Ok(file) => {
            println!("[{FILE_NAME}][18] :: open file success");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(fc) => {
                    println!("[{FILE_NAME}][24] :: create file success");
                    fc
                },
                Err(e) => panic!("[{FILE_NAME}][27] :: Problem creating the file: {:?}", e),
            },
            other_err => {
                panic!("[{FILE_NAME}][30] :: Problem opening the file: {:?}", other_err);
            }
        }
    };
}

pub fn get_file_use_unwrap_or_else(file: &str) {
    let _f = File::open(file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let fc = File::create(file).unwrap_or_else(|error| {
                panic!("[{FILE_NAME}][40] :: when create file fail: {:?}", error);
            });
            println!("[{FILE_NAME}][42] :: create file success: {}", &file);

            fc
        } else {
            panic!("[{FILE_NAME}][46] :: open file fail: {:?}", error);
        }
    });

    println!("[{FILE_NAME}][50] :: open file success: {}", &file);
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

    println!("[{FILE_NAME}][56] :: open file success: {}", &file);
}

static FILE_NAME: &str = "error_demo/index.rs";

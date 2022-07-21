use std::io;

pub fn stdin() {
    println!("[{FILE_NAME}][4] :: Please input your input.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("[{FILE_NAME}][10] :: You input: {}", input);
}

static FILE_NAME: &str = "stdin_demo.rs";

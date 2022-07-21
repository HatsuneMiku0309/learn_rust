pub fn enum_ipaddr_demo() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));
    println!("[{FILE_NAME}][12] :: {:#?}", home);
}

pub fn enum_option_demo() {
    let some_number = Some(10);
    let some_string = Some("test");

    let absent_number: Option<i32> = None;

    println!("[{FILE_NAME}][21] :: some_number: {:#?}", some_number);
    println!("[{FILE_NAME}][22] :: some_string: {:#?}", some_string);
    println!("[{FILE_NAME}][23] :: absent_number: {:#?}", absent_number);
}

#[allow(dead_code)]
pub fn enum_option1_demo() {
    #[derive(Debug)]
    enum Option1<T> {
        None,
        Some(T),
    }

    let some_number: Option1<&str> = Option1::Some("TEST");
    let absent_number: Option1<&str> = Option1::None;
    println!("[{FILE_NAME}][36] :: c_some_number: {:#?}", some_number);
    println!("[{FILE_NAME}][37] :: c_absent_number: {:#?}", absent_number);
}

static FILE_NAME: &str = "match_demo/enum_demo.rs";

pub fn unwrap_options() {
    let x = Some("value");
    let _a = x.expect("fruits are healthy"); 
    assert_eq!(x.expect("fruits are healthy"), "value");

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
}


pub fn vec_test() {
    let v = vec![1, 2, 3, 4, 5];
    // let mut third = &v[20];
    let mut third = v.get(20).unwrap_or_else(|| { &0 });
    println!("third is: {}", third);
    third = &v[4];

    println!("third is: {}", third);
    println!("v is: {:?}", v);
}
pub fn new_vec() {
    // #This is an important point#
    // Note that we added a type annotaion here.
    // Because we aren't inserting any values into this vector, Rust doesn't know what kind of elements we intend to store.
    #[allow(unused_variables)]
    let v: Vec<i32> = Vec::new();

    // More often, you'll create a `Vec<T>` with initial values and Rust will infer the type of value you want to store
    #[allow(unused_variables)]
    let mut v2 = vec![1, 2, 3];


    v2.push(5);
    v2.push(6);
    for i in 7..9 {
        v2.push(i);
    }


} // <- v/v2 goes out of scope and is freed here. Like any other `struct`, a vector is freed when it goes out of scope.


pub fn read_vec_elem() {
    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    println!("[{FILE_NAME}][26] :: The third elem is {}", third);

    match v.get(2) {
        Some(third) => println!("[{FILE_NAME}][29] :: The third elem is {}", third),
        None => println!("There is no third element."),
    }

    // The reason Rust provides these two ways to reference an element is so you can choose how the program
    // let v2 = vec![1, 2, 3, 4, 5];
    /*
        When we run this code, `[]` method will cause the program to panic because it references a nonexistent element.
        This method is `best used` when `you want your program to crash if there's an attempt to access an element past the end of the vector`.
    */
    // let does_not_exist = &v2[100];
    /*
        When ths `get` method is passed an index that is outside the vector, it returns `None` without panicking.
        You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
        Your code will then have logic to handle having either `Some(&element)` or `None`.
    */
    // let get_does_not_exist = v2.get(100);

}


fn iterating_over_the_values_in_vector() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}


static FILE_NAME: &str = "collections_demo/vec_demo.rs";

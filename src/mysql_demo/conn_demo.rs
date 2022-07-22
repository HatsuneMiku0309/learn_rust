use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Test {
    id: i32,
    name: String
}

pub fn mysql_conn_demo() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://yousee:3small?@localhost:33306/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    #[allow(unused_parens)]
    let selected_test = match conn.query_map(
        "SELECT ID id, NAME name FROM TEST1",
        | ( id, name) | { Test { id, name } },
    ) {
        Ok(val) => val,
        Err(_) => {
            println!("Fail");
            Vec::new()
        }
    };
    // let selected_test = conn.query_map(
    //     "SELECT ID id, NAME name FROM TEST1",
    //     | ( id, name) | { Test { id, name } },
    // ).expect("Fail query");

    // let selected_test = conn.query_map(
    //     "SELECT ID id, NAME name FROM TEST1",
    //     | ( id, name) | { Test { id, name } },
    // )?;

    for (i, select) in selected_test .iter().enumerate(){
        println!("index: {}, value(id): {}, value(name): {}", i, &select.id, &select.name);
    }
    println!("");
    println!("{:?}", selected_test);

    Ok(())
}

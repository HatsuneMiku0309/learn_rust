pub fn iter_demo() {
    let v1 = vec![1,2,3];
    // let mut v1_iter = v1.into_iter();
    // let mut v1_iter = v1.iter_mut();
    let mut v1_iter = v1.iter();
    v1_iter.next();
    v1_iter.next();
    // v1_iter.next();
    let a = v1_iter.next();
    let b;
    let _qaq = a.expect("test");
    let _qaq2 = a.unwrap();
    match a {
        Some(value) => { b = *value; },
        None => { panic!("gg"); }
    }
    // if let Some(value) = a {
    //     *value = 123;
    // }
    println!("b is: {}", b);
    println!("v1 is: {:?}", v1);
}

pub fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    println!("total is: {}", total);
    assert_eq!(total, 6);
}

pub fn other_iter() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("v2 is: {:?}", v2);
    assert_eq!(v2, vec![2, 3, 4]);
}

/*
 * add `#[derive(Debug)]` to `Shoe` or manually `impl Debug for Shoe`
 * binary operation `==` cannot be applied to type `Vec<Shoe>`: #[derive(PartialEq)]
 * 
 * because use assert
 */
#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

pub fn use_closure_get() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("運動鞋")
        },
        Shoe {
            size: 13,
            style: String::from("涼鞋")
        },
        Shoe {
            size: 10,
            style: String::from("靴子")
        }
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("in_my_size is: {:?}", in_my_size);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("運動鞋")
            },
            Shoe {
                size: 10,
                style: String::from("靴子")
            }
        ]
    );
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
        //     Some(self.count)
        // } else {
        //     None
        // }
    }
}

pub fn calling_next_directly() {
    let mut counter = Counter::new();
    
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

pub fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| {
            // println!("a is: {}", a);
            // println!("b is: {}", b);
            a * b
        })
        .filter(|x| {
            // println!("x is: {}", x);
            x % 3 == 0
        })
        .sum();

    println!("sum is: {}", sum);
    assert_eq!(sum, 18);
}
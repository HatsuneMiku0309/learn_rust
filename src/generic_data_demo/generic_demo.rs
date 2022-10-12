// pub fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> PointTwo<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointTwo<X2, Y2>) -> PointTwo<X1, Y2> {
        PointTwo { x: self.x, y: other.y }
    }
}

pub fn generic_point() {
    let p = Point {
        x: 5,
        y: 4
    };
    let p2 = Point{
        x: 5.0,
        y: 4.0
    };

    println!("p.x = {}", p.x());
    println!("distance_from_origin = {}", p2.distance_from_origin())
}

pub fn mixup_demo() {
    let p1 = PointTwo{ x: 5, y: 10.4 };
    let p2 = PointTwo{ x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

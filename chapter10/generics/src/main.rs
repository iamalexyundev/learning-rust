mod bad_main;
mod generic_basics;

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//Implementing for concrete types only
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

struct Coordinates<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Coordinates<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Coordinates<X2, Y2>) -> Coordinates<X1, Y2> {
        Coordinates {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    bad_main::main();
    generic_basics::main();

    let integer_coordinates = Point { x: 32, y: 32 };
    let float_coordinates = Point { x: 1.1, y: 1.1 };

    println!("Float's x: {}", float_coordinates.x());
    println!("Float's dfo: {}", float_coordinates.distance_from_origin());
    println!("Int's x: {}", integer_coordinates.x());
    println!("Int's dfo: not implemented");

    let p1 = Coordinates { x: 5, y: 10.4 };
    let p2 = Coordinates { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

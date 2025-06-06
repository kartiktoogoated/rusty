enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect = Shape::Rectangle(1.0,2.0);
    calc_area(rect);
    let circ = Shape::Circle(1.0);
    calc_area(circ);
}

fn calc_area (shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a,b ) => a * b, //Patter matching syntax
        Shape::Circle(r) => 3.14 * r * r,
    }
}
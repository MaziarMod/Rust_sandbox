struct Point {
    x: f64,
    y: f64
}

fn getPoint() {
    let point1 = Point {x: 3.0, y: 4.0};
    println!("point1 is at ({}, {})", point1.x, point1.y);
}
fn main() {
   getPoint();
}

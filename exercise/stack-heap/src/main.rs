
#![allow(dead_code)]

use std::mem;

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{ x: 0.0, y: 0.0}
}
pub fn stack_heap() {
    let point1: Point = origin(); //Stack
    let point2 = Box::new (origin()); // Heap

    println!("Stack: point1 takes {} bytes", mem::size_of_val(&point1)); // will show 16 bytes because there are two f64 varibles
    println!("Heap: point2 takes {} bytes", mem::size_of_val(&point2)); // it stores only the address which is f64 (8 bytes)

    let point3 = *point2;
    println!("Access to point2 value (point2 is the address in heap) x= {}, y= {}", point3.x, point3.y); //unboxing -> relocating from heap to stack
}

fn main() {
    stack_heap();
}

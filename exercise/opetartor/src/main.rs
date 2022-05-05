#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn main() {
    println!("===========>");
    println!(" {} power of {} = {}", 2, 8, i32::pow(2, 8));

    println!(" {} power of {} = {}", 2.5, 8, f64::powi(2.5, 8));
    println!(" {} power of {} = {}", 2.5, std::f64::consts::PI, f64::powf(2.5, std::f64::consts::PI));
    
    //bitwise
    
    println!(" {} | {} = {}", 1, 2, 1 | 2); // OR
    println!(" {} & {} = {}", 1, 2, 1 & 2); // AND
    println!(" {} ^ {} = {}", 1, 2, 1 ^ 2); // XOR
    println!(" !{} = {}", 3, !3 ); //NOR
    println!(" {} << {} = {} == 2 ^ {} = {}", 1, 10, 1 << 10, 10, 1 << 10); //Shift to the left
    println!(" {} >> {} = {}", 1, 10, 1  >> 10); // Shift to the right
    
    
    
}

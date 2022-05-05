#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn main() {

    println!("===========>");

    let age: u8 = 87; //unsigned, 8 bits, 0-255
    println!(" Age: {}", age);  

   // age = 123; //cannot assign twice to immutable variable `age`


   // u = unsigned, 0 .. 2^N-1
   // i = signed, -2^(N-1) .. 2^(N-1)-1  
   let mut id: i8 = 120; // signed, 8 bits, -128 .. 127
   println!(" ID: {} before change", id);  

   id = 127; // assgined new value
   println!(" ID: {} after change", id); 

   let deposit = 666666666; // i32 - signed - 32 bits - 4 bytes
   println!(" deposit variable {} takes {} bytes", deposit, mem::size_of_val(&deposit)); 


   // usize, isize
   /*
    isize is the same as i64 and usize is the same as u64 . usize cannot be negative and is generally used for memory addresses, positions, indices, lengths (or sizes!). isize can be negative, and is generally used for offsets to addresses, positions, indices, or lengths. 
   */

   let num : isize = 120;
   let size_of_num: usize = mem::size_of_val(&num);
   println!(" num = {} takes {} bytes, {}-bit OS", num, size_of_num, size_of_num * 8);

   let letter: char = 'X';
   println!(" {} is a char, and it size is {} byte", letter, mem::size_of_val(&letter));

   // f32, f64 IEEE754, signed
   let float_num: f32 = 3.45;
   println!(" {}, and it size is {} byte", float_num, mem::size_of_val(&float_num));

   let is_empty: bool = false;
   println!(" {}, and it size is {} byte", is_empty, mem::size_of_val(&is_empty));

}

const MY_CONSTANT: u8 = 100; // no fixed address

static mut my_static_var:i64 = 9999; // if add a mut static varible compiler will complain because it is not safe

fn main() {
    println!("MY_CONSTANT ==> {}", MY_CONSTANT);

    //println!("Static variable value {}", my_static_var); // error will be thrown
    unsafe {
        println!("Static variable value {}", my_static_var); 
    }

}

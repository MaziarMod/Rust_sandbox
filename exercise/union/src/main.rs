union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("int value in IntOf float union is equal 42");
            }

            IntOrFloat { f } => {
                println!("Float value = {}", f);
            }
        }
    }
}

fn main() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 212;

    let value = unsafe {iof.i};
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { f: 100.0 });

    // In this case, the integer value will be treated as a floating point number. And we're going to do a reinterprets cast. You basically take this memory, which is an integer, and you're going to print it out as if it were representing a floating point number.
    process_value(IntOrFloat { i: 5 });

}

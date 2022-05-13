use std::env;

fn main() {
    let args:  Vec<String> =  env::args().collect();

    println!(" Args: {:?}", args);

    // Note: enter more than one args in terminal
    println!(" Arg[1]: {}", args[1]);
}

/*
 * Example:  
 * 
 * in Terminal, type cargo run --> you will get Args: ["target/debug/cli"]
 * type cargo run hello CLI --> Args: ["target/debug/cli", "hello", "CLI"]
 * type cargo run hello CLI --> Arg[1] = hello
 */
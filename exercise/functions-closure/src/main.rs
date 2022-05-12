fn main() {
    greeting("Howdy", "Bird");
    let multiplication = multiple(250, 230);
    println!("Multiplication: {}", multiplication);

    // Closure
    let add_nums = |num1: i64, num2: i64| num1 + num2;
    println!("Closure ==> Sum: {}", add_nums(2000, 123));

    // Closure - can use outside variables
    let num3: i64 = 666;
    let add_nums2 = |num1: i64, num2: i64| num1 + num2 + num3;
    println!("Closure (use outside variables)  ==> Sum: {}", add_nums2(2000, 123));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, how is it going?", greet, name);
}

fn multiple(num1: i32, num2: i32) -> i32 {
    num1 * num2  //if we don't semicolon, it means we want to return it
}
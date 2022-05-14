fn main() {
   let x = 10.0;
   let y = 0.0;

   //Option -> two possible values -> Some(value) or None
   let result = if y != 0.0 { Some(x/y) } else { None };

   // Two way to test it -> 1. match 2. if let / while let
    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("Error: Division by Zero {} / {}", x, y)
    }

    // if the value is None, so there is no output
    if let Some(z) = result {
        println!("result (if let) = {}", z);
    }
    // if the value is None, so there is no output
    // otherwise, there is a loop and we should consider a breaking point
    while let Some(z) = result {
        println!("result (while let) = {}", z);
    }
}

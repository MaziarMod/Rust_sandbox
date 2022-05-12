fn main() {
    // Primitive str = Immutable, fixed-length somewhere in memory. you are not able to use push or push_str
    let my_str = "Hello";

    //String = Growable, heap-allocated data structure - use when you need to modify or own string data
    let mut my_growable_str = String::from("Hello!");

    // Get lenght
    println!("Length: {}", my_growable_str.len());

    my_growable_str.push('$'); // for appending one character
    println!("My Growable String: {}", my_growable_str);

    my_growable_str.push_str(" Univers \u{1F600}"); // for appending a string
    println!("My Growable String: {}", my_growable_str);

    // Capacity in Bytes
    println!("Capacity: {}", my_growable_str.capacity());

    //check if empty
    println!("Is empty: {}", my_growable_str.is_empty());

    //Contain
    println!("Contain 'Univers' {}", my_growable_str.contains("Univers"));

    //Replace
    println!("Replace 'Univers' with 'World!' ====> {}", my_growable_str.replace("Univers", "World!"));

    //Loop through string by whitespace
    for word in my_growable_str.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut str1 = String::with_capacity(5);
    str1.push('M');
    str1.push('a');
    str1.push('z');
    println!("Sting with capacity = {}", str1);

    //Assertion testing
    assert_eq!(3, str1.len()); // nothing happens because str1.len() = 3

    
}

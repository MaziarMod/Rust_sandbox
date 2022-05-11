fn main() {
    //print to console
    println!("Rust Programming...");

    //Basic formating
    println!("{} is cool and {} should go to the park", "Today", "she" );

    //Positional arrguments
    println!("{0} is cool and {1} should go to the park {0}", "Today", "she" );

    //Named arguments
    println!("{project} project will be finished {date}", project = "Go to the moon", date = "May 11, 2022");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Oct: {:o}", 25, 25, 25);

    //placeholder for debug traits
    println!("{:?}", ("test", false, 16));

    //Basic math
    println!("120 * 86 = {}", 120 * 86);

}

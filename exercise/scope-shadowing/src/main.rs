fn scope() {
    let a = 100;
    {
        let a = 666;
        println!("(Scope fn): inside a -> {}", a);
    }
    println!("(Scope fn): outside a -> {}", a);
}

fn shadowing() {
    let b = 100;
    {
        println!("(shadowing fn): inside b -> {}", b);
    }
    println!("(shadowing fn): outside b -> {}", b);
}

fn main() {
    scope();
    shadowing();
}

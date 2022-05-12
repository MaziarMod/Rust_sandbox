
fn main() {
    let temperature = 10;
    if temperature > 20 {
        println!("it is hot");
    } else if temperature < 10 {
        println!("it is cold");
    } else {
        println!("cool");
    }

    let day = if temperature > 20 {"too hot!"} else {"nice!"};
    println!("Today is {}", day);

    let mut num = 1;
    while num < 1000 {
        num *= 2;

        if num == 128 { continue; }
        println!("number(while): {}", num);
    }

    let mut num2 = 1;
    loop {
        num2 *= 2;
        println!("number(loop): {}", num2);

        if num2 == 1 << 9 {break;} 
    }

    for num3 in 1..10 {
        println!("number(for): {}", num3);
    }

    for (position, num4) in (100..110).enumerate() {
        println!("(for): Position {} --> Value {}", position, num4);
    }

    // infinit loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("From infinit loop: {}", counter);

        if counter == 8 {break};
    }
}

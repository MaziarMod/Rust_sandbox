#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}
fn main() {
    let code = String::from("Maziar!");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end()); //push_str allows to append a string to another string
                    }

                    Err(_) => continue
                }
               if entry == code {
                   state = State::Unlocked;
                   continue;
               }

               if !code.starts_with(&entry) {
                   state = State::Failed;
               }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear(); //clear the string ""
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }

        }
    }
}

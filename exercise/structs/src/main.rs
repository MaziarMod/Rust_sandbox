// Structs - used to create custom data types

// Trditional Struct
struct Point {
    x: f64,
    y: f64
}

fn get_point() {
    let mut point1 = Point {x: 3.0, y: 4.0};

    // Change the value
    point1.x = 8.5;

    println!("point1 is at ({}, {})", point1.x, point1.y);
}

//Tuple Struct
struct Color(u8, u8, u8);

fn get_color() {
    let color1 = Color(100, 100, 0);
    println!("Color1: ({}, {}, {})", color1.0, color1.1, color1.2);
}


struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    //Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String { //self is smillar "this" in other languages
        format!("{} {}", self.first_name, self.last_name)
    }

    //set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

fn get_person() {
    let mut person = Person::new("MyFirstName", "MyLastName");

    println!("Person: {} {}", person.first_name, person.last_name);
    println!("Person's Full Name: {}", person.full_name());

    person.set_last_name("Rust dev");
    println!("Changed last name: {}", person.full_name());

    // remember because we have tuple we have to use debug trait "{:?}" here
    println!("Person to tuple: {:?}", person.to_tuple());
    
}

fn main() {
   get_point();
   get_color();
   get_person();
}

// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// Tuple Struct
// struct Color(u8, u8, u8);

// 
struct Person {
    first: String,
    last: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person { 
            first: first.to_string(), 
            last: last.to_string() 
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }

    //  Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first, self.last)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // c.red = 200;
    
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // let mut c = Color(255, 0, 0);
    // c.0 = 200;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Jasdeep", "Singh");
    println!("Person: {}", p.full_name());
    // p.first.push_str(" Singh");
    p.set_last_name("Dhillon");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple())
}


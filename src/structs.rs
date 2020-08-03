// traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }


//tuple struct
// struct Color(u8, u8, u8);


// 
struct Person {
    first_name: String,
    last_name: String
}


//stores functions for person
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name) 
    }
}

pub fn run() {
    // used to create custom data types
    
    //use with traditional
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // c.red = 200;

    // println!("Color: {}, {}, {}", c.red, c.green, c.blue);


    //tuple style
    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("color {} {} {}", c.0, c.1, c.2);


    //call person
    let mut p = Person::new("Sam", "Gieseker");
    p.first_name = String::from("Sammy");
    p.first_name = "samuel".to_string();
    println!("first name {} last name {} like a sprained ankle", p.first_name, p.last_name);
    //calls fullname fn
    
    p.set_last_name("Williams");
    println!("name {}", p.full_name());
    println!("name tuple {:?}", p.to_tuple());
}
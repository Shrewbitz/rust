// tuples group together values of different types
// max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Sammy", "Michigan", 26);

    println!("{} loves {} and is {} years old", person.0, person.1, person.2);
}
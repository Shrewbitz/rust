pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let friend = false;

    if age >= 21 && check_id || friend {
        println!("What would you like to drink?");
    } else if age <21 && check_id {
        println!("Sorry you have to leave");
    } else {
        println!("Ill need your ID");
    }

    //short if

    let is_of_age = if age >= 21 {true} else {false};
    println!("{}", is_of_age)
}
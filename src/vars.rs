pub fn run() {
    let name = "brad";
    let mut age = 29;
    println!("my name is {} and I am {}", name, age);
    age = 30;
    println!("my name is {} and I am {}", name, age);


    //usually use let, const must declare type
    const ID: i32 = 001;
    println!("ID: {}", ID) ;

    // multiple variables
    let (my_name, my_age) = ("Sammy", 26);
    println!("{}{}", my_name, my_age);
}
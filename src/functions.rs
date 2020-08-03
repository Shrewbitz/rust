pub fn run() {
    greeting("hello", "Jim");

    //bind function values to variables
    let get_sum = add(5, 5);
    println!("total {}", get_sum);

    
    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 +n3 ;
    println!("ctotal {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name)
}


// if we dont use a semi colon it means we want to return that
fn add(num1: i32, num2: i32) -> i32 {
    // return num1 + num2;
    num1 + num2
}
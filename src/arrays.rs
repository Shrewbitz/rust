//fixed length

use std::mem;

pub fn run() {
    // let numbers: [i32; 5] = [1,2,3,4,5];

    // println!("{:?}", numbers);

    // let numbers: [i32; 5] = [1,2,3,4,5];

    // println!("single {}", numbers[0]);

    let mut numbers: [i32; 6] = [0,1,2,3,4,5];
    numbers[2] = 7;
    println!("{:?}", numbers);
    //arrays are stack allocated

    //get how much sspace
    println!("array occupies {} bytes", mem::size_of_val(&numbers) );

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice)

}
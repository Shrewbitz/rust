//vectors are resizable arrays
pub fn run() {
    let mut numberz: Vec<i32> = vec![0, 1, 2, 3, 4];


    numberz[2] = 20;

    numberz.push(12);
    numberz.pop();

    // loop using in

    for x in numberz.iter() {
        println!("number {}", x);
    }
    // mutate loop
    for x in numberz.iter_mut() {
        *x *= 2;
        
    }



    // println!("vector 2 is {:?}", std::mem::size_of_val(&numberz));
    println!("vector 2 is {:?}", numberz);

}
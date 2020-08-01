//primitive = fixed length

//String growable heap allocated data structure

pub fn run() {
    //primitive
    let hello = "hello";
    
    //mutable string
    let mut hello2 = String::from("hey ya! ");
    hello2.push('W');
    hello2.push_str("orld");

    println!("Capacity: {}", hello2.capacity());
    // hello2.is_empty() false // hello2.contains("World") true // hello2.replace("World, there") 

    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('q');
    s.push('p');


    //assertion testing USEFUL
    assert_eq!(10, s.capacity());

    println!("{}", s);
    // println!("{1} {0}", hello, hello2 );
    // println!("length: {}", hello2.len());
    // push a char or push a str
}
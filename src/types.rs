// u8 i8 u16 i16 u32 i32 u64 i64 u128 i128(bits they take up)
//u means non negative 4 - 5 = 1 i think
// i32 most common
//floats f32 f64   
// boolean bool
//characters char
//tuples ( it is a list)
//arrays (fixed length use vectors for a growable array)

pub fn run() {
    //default = i32
    let i = 1; 

    //default f64
    let y = 2.5;

    //explicit type
    let z: i64 = 4545434343;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    //bool
    let is_active = true;

    let is_greater: bool = 10 < 5;

    //char hast to be one char or a unicode symbol
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (i, y, z, is_active, is_greater, a1, face)); 
}
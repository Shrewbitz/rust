// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointerref;
// mod structs;
mod enums;

fn main() {
    enums::run();
}
// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello Sammy!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

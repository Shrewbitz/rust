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
// mod enums;
// mod basics;
mod leet_code;

fn main() {
    // basics::strings::run();
    // leet_code_test::sum3(vec![-1,3,5,-2,9,10], 16);
    // leet_code::leet_code_test::sum3(vec![1,1,1,1], -100);
    // leet_code::alien_dictionary::run(vec!["jyr","e"],"ygdehxlwjqifpabnzkrmtusovc".to_string());
    // leet_code::alien_dictionary::run(vec!["word","world","row"], "worldabcefghijkmnpqstuvxyz");
    leet_code::alien_dictionary::run(vec!["apple","app"], "abcdefghijklmnopqrstuvwxyz".to_string());
}
//3,5,-2

// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello Sammy!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

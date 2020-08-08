// enums are types that have a few definite values

enum Movement {
    //variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //peform action depending on info  /  like a switch case
    match m {
        Movement::Up => println!("moves up"),
        Movement::Down => println!("moves down"),
        Movement::Left => println!("moves left"),
        Movement::Right => println!("moves right")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
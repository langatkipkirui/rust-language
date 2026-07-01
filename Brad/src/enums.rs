// Enum are types which have few definite values

enum Movement {
    Up,
    Down,
    Left,
    Right,
}
fn move_avatar(m: Movement) {
    // perorm action deopending on info

    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Right => println!("Avatar moving right"),
        Movement::Left => println!("Avatar moving left"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

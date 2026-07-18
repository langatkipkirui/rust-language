pub fn run() {
    let rect = (200, 500);

    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        user_name: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user_one = User {
        active: true,
        user_name: String::from("johndoe"),
        email: String::from("johndoe@gmail.com"),
        sign_in_count: 2,
    };

    user_one.email = String::from("johndoe1@gmail.com");
    println!("User email {}", user_one.email);

    fn build_user(email: String, user_name: String) -> User {
        User {
            active: true,
            email,
            user_name,
            sign_in_count: 1,
        }
    }
    // Create instances from other instances
    let user_two = User {
        email: String::from("usertwo@gmail.com"),
        ..user_one
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let back = Color(0, 0, 0);
    let white = Color(255, 255, 255);
}

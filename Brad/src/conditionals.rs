// used to check the condition of something and

pub fn run() {
    let age = 21;
    let check_id = false;
    let knows_person_age = true;

    // if else
    if (age >= 21 && check_id) || knows_person_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Bartender: I will need to see your ID");
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {:?}", is_of_age)
}

pub fn run() {
    let age: u8 = 24;
    let check_id = true;
    let knows_age = true;

    if age >= 21 && check_id || knows_age {
        println!("bar tender said what you like to drink?");
    } else if age < 21 && check_id {
        println!("Sorry you have to leave.");
    } else {
        println!("Need to see your id");
    }

    // Short hand
    let is_of_age = if age >= 21 {true} else {false};
    println!("IS_OF_AGE: {}", is_of_age);
}
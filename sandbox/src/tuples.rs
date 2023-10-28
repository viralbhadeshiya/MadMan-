pub fn run() {
    let person: (&str, &str, i8) = ("vir", "India", 24);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    let person_1 = (("test", "test", "test"), "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test12");
    println!("{:?}", person_1.0);
}
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let cmd = args[1].clone();

    let name = "Brad";
    let status = 100;
    if cmd == "Hello" {
        println!("Hi {}, how are you?", name);
    } else if cmd == "Status" {
        println!("Status is {}%", status);
    } else {
        println!("that is not valid command");
    }

}
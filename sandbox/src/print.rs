pub fn run() {
    // Normal log
    println!("Hello from other file");

    // Basic format
    println!("{} is from {}", "viral", "India");

    // Positional args
    println!("{0} is from {1}, and {0} like to {2}", "viral", "India", "box");

    // Names args
    println!("{name} likes to play {activity}", name="viral", activity="football");

    // Placeholder traits
    println!("Binary: {:b}, octa: {:o}, hex: {:x}", 10, 10 ,10);

    // Placeholder debug trait
    println!("{:?}",(12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10+10);
}
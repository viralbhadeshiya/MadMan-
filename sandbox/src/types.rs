use std::i32;

pub fn run() {
    let x = 1;
    let y = 3.14;
    let z: i64 = 2384378543643563457;

    println!("i32 MAX: {}", i32::MAX);
    println!("i64 MAX: {}", i64::MAX);

    // boolean
    let is_active = true;
    let is_greater: bool = 5<3;

    // char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
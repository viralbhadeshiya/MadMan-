use std::mem;

pub fn run() {
    let mut num: [i32; 4] = [0, 1, 2, 3];
    println!("{:?}", num);
    
    // Single vl
    println!("first: {}, len: {}, mem: {}", num[0], num.len(), mem::size_of_val(&num));
    
    num[2] = 20;
    println!("{:?}", num);
    
    // Slices
    let slice: &[i32] = &num[1..4];
    println!("slice: {:?}", slice);
}
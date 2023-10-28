use std::mem;

pub fn run() {
    let mut num: Vec<i32> = vec![1, 2, 3];
    num.push(5);
    num.push(6);
    num.push(7);

    num.pop();
    println!("{:?}", num);
    
    // Single vl
    println!("first: {}, len: {}, mem: {}", num[0], num.len(), mem::size_of_val(&num));

    // Loop
    for x in num.iter() {
        println!("num: {}", x);
    }

    // Loop & mutate
    for x in num.iter_mut() {
        *x *= 2;
    }
    println!("new: {:?}", num);
}
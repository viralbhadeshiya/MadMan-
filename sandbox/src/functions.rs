pub fn run() {
    greeting("hello", "Jane");

    // Bind function to var
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Closure
    let n3: i32 = 6;
    let add_num = |num1: i32, num2: i32| num1 + num2 + n3;
    println!("new sum: {}", add_num(3, 3)); 
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
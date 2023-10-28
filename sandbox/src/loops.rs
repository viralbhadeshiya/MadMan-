pub fn run() {
    let mut count = 0;
    
    // loop {
    //     count += 1;
    //     println!("num count: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While (Fizzbuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FIZZBUZZ");
        } else if count % 3 == 0 {
            println!("FIZZ");
        } else if count % 5 == 0 {
            println!("BUZZ");
        } else {
            println!("Count: {}", count);
        }

        count += 1;
    }

    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FIZZBUZZ");
        } else if x % 3 == 0 {
            println!("FIZZ");
        } else if x % 5 == 0 {
            println!("BUZZ");
        } else {
            println!("Count: {}", x);
        
        }
    }
}
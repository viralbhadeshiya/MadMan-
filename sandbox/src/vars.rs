pub fn run() {
    let name = "vir";
    let mut age = 23;

    println!("My name is {} and I am {}", name, age);

    // birthday happens
    age = 24;

    println!("My name is {}, and now I am {}", name, age);

    // Define const
    const ID: i32 = 001;
    println!("Id: {}", ID);

    // Assign multiple var
    let (my_name, my_age) = ("Vir", 24);
    println!("My name is {}, and my age is {}", my_name, my_age);
}
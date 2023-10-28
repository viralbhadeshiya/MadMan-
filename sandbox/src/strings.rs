pub fn run() {
    let prim = "coast";
    let mut non_prim = String::from("guard");

    // Lenght
    println!("prim len: {}, non-prim len: {}", prim.len(), non_prim.len());

    non_prim.push(' ');
    non_prim.push_str("good change");

    println!("value: {}, capacity: {}, is_empty: {}, contain: {}", non_prim, non_prim.capacity(), non_prim.is_empty(), non_prim.contains("good"));

    for word in non_prim.split_whitespace() {
        println!("word: {}", word);
    }


}
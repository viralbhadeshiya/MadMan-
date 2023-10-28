pub fn run() {
    // Prim array 
    // let arr1 = [1, 2, 3];
    // let arr2 = arr1;

    // println!("Values arr1: {:?}, arr2: {:?}", arr1, arr2);

    // Vector 
    let arr1: Vec<i32> = vec![1, 2, 3];
    let arr2 = &arr1;

    println!("Values arr1: {:?}, arr2: {:?}", &arr1, arr2);
}
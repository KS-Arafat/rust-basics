pub fn vectors() {
    // Vector
    // Vector stores valeus in the Heap but
    // the actual structure of the vector is in the Stack
    // Like pointer, capacity, length
    println!("[Vector]:");
    let mut vec_str = vec![];
    vec_str.push("R");
    vec_str.push("u");
    vec_str.push("s");
    vec_str.push("t");
    vec_str.push("💙");

    for l in vec_str.iter() {
        print!("{}", l);
    }
    println!();
}

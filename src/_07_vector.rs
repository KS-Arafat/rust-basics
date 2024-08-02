pub fn vectors() {
    // Vector
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

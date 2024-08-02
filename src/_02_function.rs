pub fn functions() {
    // Functions
    println!("{:?}", add(0.2, 0.3));

    // [Destructure]
    let mut v1 = 55;
    let mut v2 = 11;
    println!("Before Swap: {:?}", [v1, v2]);
    (v1, v2) = swap(v1, v2);
    println!("After Swap: {:?}", [v1, v2]);
}

// Must explicitly declare the return type of
// the function with arrow "->"
fn add(x: f32, y: f32) -> f32 {
    return x + y;
}

// [Destructure]
fn swap(x: i32, y: i32) -> (i32, i32) {
    // Idiomatic way of returning values from function, if, match
    // or any scope block by typing expression without semicolon at  the end
    (y, x) // No semicolon at the end, returned as value
}

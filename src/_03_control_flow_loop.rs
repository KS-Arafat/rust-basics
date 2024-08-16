pub fn control_flow() {
    // Control Flow
    println!("[Control Flow]:");
    if 5 < 5 {
        println!("No Round Braket Required for Control Flow");
        // You can use brackets if you want
    } else if 3.3f32 != 3.4 {
        println!("Compararision has to be between same types and sizes")
    } else {
        println!("Also can't skip braces like C for one line");
    }

    // shorthand if else
    // if expr {`if true`} else {`if false`};
    let is_even = if 5 % 2 == 0 { true } else { false };
    println!("Is 5 even: {}", is_even);

    // Switch Case
    // This can return values but
    // all reaturn value has to be same type like C switch
    let x = 5;
    let switch_return = match x {
        0 => "0".to_owned(),
        1..=9 => "<10".to_owned(),
        // Can get the actual value of 'x' if it matches the case
        matched @ 10..=50 => format!("<50 Number: {}", matched),
        _ => ">10".to_owned(),
    };
    println!("{}", switch_return);
}

pub fn loops() {
    // Loops
    println!("[Loops]:");
    let mut i = 0;

    // Easy Loop
    println!();
    let loop_return = loop {
        i += 1;
        if i % 2 == 0 {
            print!("{} ", i);
        }
        if i == 8 {
            break "Break in Loop Can Return Value";
        }
    };
    println!("{}", loop_return);
    i = 0;
    // Old While
    println!();
    while i < 3 {
        print!("{} ", i);
        i += 1;
    }

    // Custom For
    // .. operator creates iterators from left operand to
    // right operand - 1 and ..= operator does the whole thing

    // x..y => x,....,y-1 & x..=y => x,....,y

    println!();
    // Loops through 0 to 2
    for i in 0..3 {
        print!("{} ", i);
    }
    println!();
    // Loops through 0 to 3
    for i in 0..=3 {
        print!("{} ", i);
    }
    println!();
}

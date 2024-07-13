fn main() {
    println!("Hello, world!");

    // Variables
    let a = true;
    let b = 1; // i32
    let c = 1i8;
    let d = 1.1; // f64
    let e = 1.1f32;
    let f = 'a';
    let g = "Hello, world!";
    let h = '😀';
    let i = [1, 3];
    let j = (a, b, c, d, e, f, g, h, i); //tuple
    println!("{:?}", j);

    // Type conversions
    println!("{}", 'a' as i8);
    println!("{}", 5 as f32);
    println!("{}", -5.5 as u16);

    // Constants
    const ONE_THIRD: f32 = 0.333333333;
    const TWO_THIRD: f32 = 2.0 / 3.0; // Decimal point is neccessary
    println!("{:?}", [ONE_THIRD, TWO_THIRD]);

    // Functions
    println!("{:?}", add(ONE_THIRD, TWO_THIRD));

    // Destructure
    let mut v1 = 55;
    let mut v2 = 11;
    println!("Before Swap: {:?}", [v1, v2]);
    (v1, v2) = swap(v1, v2);
    println!("After Swap: {:?}", [v1, v2]);

    // Control Flow
    if 5 < 5 {
        println!("No Round Braket Required for Control Flow");
        // You can use brackets if you want
    } else if 3.3f32 != 3.4 {
        println!("Compararision has to be between same types and sizes")
    } else {
        println!("Also can't skip braces like C for one line");
    }

    // Loops

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

    // Switch Case
    // This can return values but
    // all reaturn value has to be same type like C switch
    let x = 5;
    let switch_return = match x {
        0 => "0".to_owned(),
        1..=9 => "<10".to_owned(),
        matched @ 10..=50 => format!("<50 Number: {}", matched),
        _ => ">10".to_owned(),
    };
    println!("{}", switch_return);
}

fn add(x: f32, y: f32) -> f32 {
    return x + y;
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    // Idiomatic way of returning values from function, if, match
    // or any scope block by typing expression without semicolon at  the end
    (y, x) // No semicolon at the end, returned as value
}

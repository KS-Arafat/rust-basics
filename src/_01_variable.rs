pub fn variables() {
    // Variables
    println!("[Variable]:");
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
}

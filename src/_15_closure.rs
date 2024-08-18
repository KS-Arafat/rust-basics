pub fn anon_fn() {
    println!("[Basic Closure]:");

    // Like C lamda functions, but with borrow ownership semantics
    // Below closure is basic one but very expressive implementation

    // Commented out clousers are same
    // let add_one_v1 = |x: u32| -> u32 { x + 1 };
    // let add_one_v2 = |x: u32| x + 1;

    let x_pow_n = |x: u32, n: u32| -> u32 {
        let mut xn = 1;
        if x == 0 {
            return 1;
        }
        for _ in 1..=n {
            xn *= x;
        }
        xn
    };
    let two = 2;
    let four = 4;
    let two_pow_four = x_pow_n(two, four);
    // Unlike functions, closures only borrows the arguments
    // So we can safely access them in the closure
    println!("{} to the power {} is {}", two, four, two_pow_four);
    // If we have passed them to function without &(borrow),
    // we wouldn't be able to print them

    let s1 = String::from("S1");
    let s2 = String::from("S2");

    let not_a_thief = || println!("Moved by Borrow: {s1}");
    not_a_thief();
    thief(s2);
    println!("{s1} Still Available");
    // println!("{s2}");// borrow of moved value: `s2`
    // Still passing arguments to closure is moved by ownership
    // it only borrows when accessing scope variables

    // Closure and Vector

    let mut primes = vec![2, 3, 5];
    let mut add_next_prime = || primes.push(7);
    // See we had to use `let mut` because closures are treated as values
    // so that it can mutate its' captured variables
    add_next_prime();
    println!("Primes: {primes:?}");

    // Move Closure works like regular function, but whatever variables
    // are moved to closure will be moved by ownership and will not be accessible
    let not_not_a_thief = move || println!("Moved by Ownership: {s1}");
    not_not_a_thief();
    // println!("{}", s1); // borrow of moved value: `s1`
    // More on this type closure un multi-threading
}

fn thief(s: String) {
    println!("Moved by Value: {s}");
}

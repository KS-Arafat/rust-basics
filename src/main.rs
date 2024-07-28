use std::any::type_name;

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

    // [Destructure]
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
    println!();

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
    // Done "https://tourofrust.com/21_en.html"

    // Struct & enum

    // [Struct]: has to be initalized when assigning to an object
    let obj = MyStruct {
        x: 420,
        y: 6.9,
        z: String::from("From Structure"),
        human_bool: Hbool::Yes,
        // [Enum]: Path seperator(::) is used to get Hbool value
        // See we use :: for getting value from enum same with String::from
        // it's because they are statically defined
    };

    // [Enum]: Can't use it in control block to compare enum so use match instead
    let human_boolean = match obj.human_bool {
        Hbool::Yes => "Yes",
        Hbool::No => "No",
    };
    println!("{} {} {} {}", obj.x, obj.y, obj.z, human_boolean);

    // [Struct]: Array like access
    let obj2 = TupleStruct(420, 6.9, Hbool::No);

    // passing vailable by reference using & like C
    println!("{} {} {}", obj2.0, obj2.1, get_human_boolean(&obj2.2));

    // [Generic]: Template or Generics
    // Pass any type of vailable to structure and
    // it wil be morped to that type
    let auto_type_v = Generics {
        morph_type_variable: "Type is not static here",
    };
    // Also can be used in function but very complex to handle generics
    generic_func(auto_type_v.morph_type_variable);

    // [Error Handling]: Error handling functions will always return "Result" type and
    // We have to match it with "OK" and "Err"
    match is_even(11) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
    // Another Easy way to handle code above is to declare "main" function return type
    // as "Result" type like "fn main()->Result<T,E>" then we can use
    // let v = is_even(12)?; // V will contain success or error value

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

    // Next Up Ownership "https://tourofrust.com/43_en.html"

    // Cahpter 05
    // [Ownership]:
    // Owner Borrow
    let owner1 = "I can have one owner".to_string();
    // here owner1 is owner, Ownership can moved
    let owner2 = owner1; // Ownership changed to owner2

    // println!("{}", owner1); // error[E0382]: borrow of moved value: `owner1`
    // Above code would show error as the owner was changed
    // Thus first owner is invalid and unusable
    println!("{}", owner2);

    // Not all varaible works like this
    let own1 = "I can have many owners";
    let own2 = own1;
    println!("{} {}", own1, own2);

    // Here own1 and own2 are referencing to the same string literal
    // But cant be used for ownership. Also it is immutable

    // We can have multiple owners by referencing to the same value like this
    // Borrowing ownership using Reference
    let owner3 = &owner2;
    println!("{} {}", owner2, owner3);
    // No Error as ownership hasnt moved to owner3 instead owner3 is referencing to owner2

    // Passing owner to another function as owner will invalidate
    // the ownership of the owner

    invalidate_ownership(owner2); // Passed as Owner
    println!("Ownership moved to function Scope");
    // println!("{}", owner2);
    // Error as ownership move to function scope

    // Also we cant use
    // println!("{}", owner3);
    // As `owner3` is referencing to owner1, we cannot pass ownership to the function
    // `owner3` is automatically dropped after the last time it was called
    // In this case before we called `invalidate_ownership` function, reference was dropped

    // [Explicit Lifetime]:
    // Say we have a funtion that takes two references and returns a reference
    // without cloning or creating a new owned value, we have to return specific value
    // but we can't do that normally passing references as Rust can't infer
    // which reference should live long enough to be used

    let s1 = "Text01";
    let longer_string;
    {
        let s2 = "Text 02";
        longer_string = explicit_lifetime(&s1, &s2);
    } // [LifeTime]: instead of dropping the reference of s2 here

    // Here if s2 is longer then lifetime of longer_string reference
    // will be dropped outside of the scope thus can't be used
    println!("{}", longer_string);
    // [LifeTime]: reference of s2 and s1 will be dropped here because of explicit lifetime

    // Also there is 'static lifetime which will outlive
    // all references till the end of program
    // But this is unsafe and can only be used inside `unsafe` block
    static mut _IMMORTAL: &'static str = "I will outlive";
    unsafe {
        _IMMORTAL = "I will outlive all";
        println!("{}", _IMMORTAL);
    }

    // Chapter 5 Done
    // [Next]: https://tourofrust.com/59_en.html
}

fn add(x: f32, y: f32) -> f32 {
    return x + y;
}

// [Destructure]
fn swap(x: i32, y: i32) -> (i32, i32) {
    // Idiomatic way of returning values from function, if, match
    // or any scope block by typing expression without semicolon at  the end
    (y, x) // No semicolon at the end, returned as value
}

// [Struct]: C struct
struct MyStruct {
    x: i32,
    y: f32,
    z: String,
    human_bool: Hbool,
}
// Tupple Structure
struct TupleStruct(i32, f32, Hbool);

// [Enum]
enum Hbool {
    Yes,
    No,
}

// [Enum]: Takes enum refrence as argument unlike C function arg,
// its more like TS function args with & of course as reference
fn get_human_boolean(enm_bool: &Hbool) -> &str {
    match enm_bool {
        Hbool::Yes => "Yes",
        Hbool::No => "No",
    }
}

// [Generic]:
struct Generics<T> {
    morph_type_variable: T,
}

// [Generic]: Takes `T` type as argument and get the data type
fn generic_func<T>(_v: T) {
    let v_type = type_name::<T>();
    println!("Typeof {}", v_type);
}

// [Error Handling]: Function has to return Result type vaiable which has generics
fn is_even(i: i32) -> Result<String, String> {
    if i % 2 == 0 {
        Ok("Is Even".to_string())
    } else {
        Err("Not Even".to_string())
    }
}

// [Ownership]: This Function assign the ownership to _s and
// ownership is dropped after end of the function scope
fn invalidate_ownership(_s: String) {}

// [Explicit Lifetime]:
fn explicit_lifetime<'elt1, 'elt2: 'elt1>(s1: &'elt1 str, s2: &'elt2 str) -> &'elt1 str {
    // <'elt1, 'elt2: 'elt1>
    // Here 'elt2:'elt1 tells Rust compiler that 'elt2 must outlive 'elt1
    // Or else 'elt2 might just die
    if s1.len() < s2.len() {
        s2
    } else {
        s1
    }

    // return s1.to_string();
    // This will avoid explicit lifetime all together
    // but this will create performance overhead for allocating memory(wasted memory)
}

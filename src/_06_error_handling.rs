// [Error Handling]: Function has to return Result type vaiable which has generics
fn is_even(i: i32) -> Result<String, String> {
    if i % 2 == 0 {
        Ok("Is Even".to_string())
    } else {
        Err("Not Even".to_string())
    }
}

pub fn error_handling() {
    // [Error Handling]: Error handling functions will always return "Result" type and
    // We have to match it with "OK" and "Err"
    println!("[Error Handling]: ");
    match is_even(11) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
    // Another Easy way to handle code above is to declare "main" function return type
    // as "Result" type like "fn main()->Result<T,E>" then we can use
    // let v = is_even(12)?; // V will contain success or error value
}

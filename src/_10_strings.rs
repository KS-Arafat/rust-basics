pub fn strings() {
    // [String]:
    // String literals are always unicodes so special characters like emojis are allowed
    // Fixed Size String(&str) literals have time complexity of O(n) in worst case for searching
    // We can have multiple lines in same variable  using '\' at the end
    println!("[String]:");
    let s = "This is a string \
    This 2nd line in One quotation✅";
    println!("{}", s);

    // Also we can assign raw string using "r#" to start and "#" in the end
    // Here newlines, tabs and spaces are included in the variable so mind the code formatting
    let raw_s = r#"
        <div>
            This is a raw string
        </div>
        "#;
    println!("{}", raw_s);

    // String slicing
    // We have python like process for slicing strings, here 0..=5 means 0 to 5 (including 5)
    // and & is used to borrow the string by creating refernce to the value
    println!("{}", &raw_s.trim()[0..=5]);

    // String iteration and functions
    // To iterate over string we have to make string to chars

    for ch in s.chars() {
        print!("{}::{} ", ch, ch as u32);
    }
    println!("\nCharacter::Ascii Value");

    // String to i32, it has to be i32 cause all characters are unicode
    let number_str = "69420";
    let extracted_number = number_str.parse::<i32>().expect("Parse error");
    println!("{} {}", number_str, extracted_number);

    // Join and concatenate
    let mut array_str = vec![] as Vec<&str>;
    array_str.push("Rust");
    array_str.push("is");
    array_str.push("Fun");
    println!("{}\n{}", array_str.join(" "), array_str.concat());

    // Formatting Strings
    let emoji = "🐱‍👤";
    let formated_str = format!("Some emogi: {} and some i32: {}", emoji, extracted_number);
    println!("{}", formated_str);
}

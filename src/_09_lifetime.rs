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
pub fn expl_lifetime() {
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
}

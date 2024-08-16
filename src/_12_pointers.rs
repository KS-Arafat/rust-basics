struct DummyStt {
    value: i32,
}
pub fn pointer() {
    println!("[Pointers]: ");
    let mut a = 69;
    let mut b = &a;
    let c = 96;
    // Can't change 'a' beacuse 'a' is borrowed by 'b'
    // a = 64; // Error: cannot assign to `a` because it is borrowed
    println!("a:{} b:{}", a, b);
    b = &c;
    a = 64;
    println!("a:{} b:{}", a, b);

    // we can't print raw pointers as it is, but we can change type to usize
    // usize and u32 or u64 quite similar but in different architectures size of memory
    // is different, so usize is guaranteed to hold the memory size
    let raw_addr = b as *const i32 as usize;
    println!("{}", raw_addr);
    // * is used to de-reference the ref
    let refx2 = &&a;
    let obj_p = DummyStt { value: 56 };
    let refx3 = &&&obj_p;
    // even if object is referenced 3 times, we can directly work with the members with '.' when dealing with struct
    let deref_drct = refx3.value + 3;
    // but we can't directly do mathematical operations to with referenced variables
    let derefx2 = **refx2 + 2;
    println!("{} {}", deref_drct, derefx2);
}

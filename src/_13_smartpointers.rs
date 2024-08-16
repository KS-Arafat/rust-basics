// Smart pointer deals with problems with low level pointers in C/C++ where pointers
// have to be freed by the user. If not there will be memory leaks and security issues

use std::rc::Rc;

struct Dummy {
    value: i32,
}

pub fn smart_p() {
    // Rust has some smart pointers that has there own functionalities.

    // [Box]: Straightforward, simple
    // Box is used to store pointers in the heap rather than in stack
    let heap_str = Box::new("This string is on the heap");
    println!("[Box]: {}", heap_str);

    // When to use Box
    /*
     * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
     * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
     * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
     */

    // [Rc<T>] Reference Counted Smart Pointer
    // Until now we had only single owner for all types of variable
    // Either we had to return ownership to the original owner or we wouldn't able to use it

    // With RC pointer we can have multiple owners
    let rc_p = Rc::new(Dummy { value: 69 });
    println!("[RC]: Reference Counter");
    println!("Mem Addr of Main: {}", &rc_p.value as *const i32 as usize);
    // Rc::clone creates a new reference to the main address
    // we can test that in borrow_mem() function
    // Rc::strong_count() returns the number of clone references of main address
    println!("RC after impl: {}", Rc::strong_count(&rc_p));
    let _rc_clone1 = Rc::clone(&rc_p);
    println!("RC after Clone 1: {}", Rc::strong_count(&rc_p));
    let _rc_clone2 = Rc::clone(&rc_p);
    println!("RC after Clone 2: {}", Rc::strong_count(&rc_p));
    let rc_clone3 = Rc::clone(&rc_p);
    borrow_mem(rc_clone3);
    // Rc::strong_count() will increment, until it reaches borrow_mem()
    // because clone borrowed but never returned thus goes out of scope
    // And the address of the clone and the main address is same
    println!("RC after Clone 3: {}", Rc::strong_count(&rc_p));

    // There are some other smart pointer structure which are used in multithreading
    // shared rss and unsafe blocks. More on those later
}

fn borrow_mem(_obj: Rc<Dummy>) {
    // This function borrows but doesn't returns the ownership
    // Thus reference goes out of scope invalidating the pointer passed as argument
    println!("Mem Addr of Clone: {}", &_obj.value as *const i32 as usize);
}

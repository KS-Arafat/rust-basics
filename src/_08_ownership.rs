// [Ownership]: This Function assign the ownership to _s and
// ownership is dropped after end of the function scope
fn invalidate_ownership(_s: String) {}

pub fn ownership() {
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
}

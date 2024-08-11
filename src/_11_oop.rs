// General Structure
struct Animal {
    animal_type: String,
    animal_call: String,
    // Public Field
    pub children: i32,
}

// [Encapsulation]
// To add function or encapsulate methods with the structure
// we have to use implementation block (impl)
impl Animal {
    // we have to pass &self unmutable refernce of the instance
    fn speak(&self) -> String {
        format!("{} {}s", self.animal_type, self.animal_call)
    }
}

// [Encapsulation]
pub fn encapsulation() {
    let mut animal_instance = Animal {
        animal_type: "🐮".to_string(),
        animal_call: "Moo".to_string(),
        children: 0,
    };
    // Even if there is public field in the instance
    // we can't use it without "let mut" to make it mutable
    animal_instance.children = 10;
    animal_instance.speak();
}

// [Polymorphism]
// "trait" is  used to create skeletons of structures
trait Countchildren {
    fn get_children(&self) -> String;

    // Also we can implement method in trait
    // but it can only access members of its own trait
    // We can overwrite this in impl block if we want
    fn loveofmother(&self) -> String {
        format!("She loves her {}", self.get_children())
    }
}

// Syntax: impl [trait] for [Struct]{ ... }
impl Countchildren for Animal {
    fn get_children(&self) -> String {
        format!("{} children", self.children)
    }
    // implementing trait method in impl block will overwrite method in trait
    // fn loveofmother(&self) -> String {}
}

pub fn polymorphism() {
    let cat_animal = Animal {
        animal_type: "😽".to_string(),
        animal_call: "Meow".to_string(),
        children: 4,
    };
    println!("{}", cat_animal.speak());
    println!(
        "{} has {}\n{}",
        cat_animal.animal_type,
        cat_animal.get_children(),
        cat_animal.loveofmother()
    );
}

// dynamic and static dispatch

// If we know the root struct or know the type (Animal) then we can use static dispatch
// in this dispatch, we can access all the methods that are implemeted with the struct
fn static_dispatch(animal: &Animal) {
    println!("{}", animal.speak());
}

// And we don't know the direct type, we can use the trait as the type with "dyn" keyword
// But we can only use the method that are defined in that trait
// it is slightly slower that static dispatch
fn dynamic_dispatch(animal: &dyn Countchildren) {
    println!("she has {}", animal.get_children());
}

pub fn dispatch() {
    let bird = Animal {
        animal_type: "🐦".to_string(),
        animal_call: "whistle".to_string(),
        children: 0,
    };
    static_dispatch(&bird);
    dynamic_dispatch(&bird);
}

// [Box]

struct User {
    name: String,
}

fn iterating_vector(v: &Vec<Box<User>>) {
    for user in v {
        println!("{}", user.name);
    }
}

// Generally when compiler allocates values on the stack, but we know stack is not for
// big dataset and inefficient with it. Thats where Box allocates values on the heap
// and Box is a pointer to "User" struct not the actual Struct so moving it is efficient
pub fn box_stack() {
    let mut userlist = vec![] as Vec<Box<User>>;
    userlist.push(Box::new(User {
        name: String::from("Adam"),
    }));
    userlist.push(Box::new(User {
        name: String::from("Nuh"),
    }));
    userlist.push(Box::new(User {
        name: String::from("Lut"),
    }));
    // If we have passed regular vector then it would pass all the "User" struct to function
    // and it is bad for large data structures.
    // As Box is only pointer address its will be effiecient for passing large data structures
    iterating_vector(&userlist);
}

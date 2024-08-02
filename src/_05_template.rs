use std::any::type_name;

// [Generic]:
struct Generics<T> {
    morph_type_variable: T,
}

// [Generic]: Takes `T` type as argument and get the data type
fn generic_func<T>(_v: T) {
    let v_type = type_name::<T>();
    println!("Typeof {}", v_type);
}
pub fn generics() {
    // [Generic]: Template or Generics
    // Pass any type of vailable to structure and
    // it wil be morped to that type
    let auto_type_v = Generics {
        morph_type_variable: "Type is not static here",
    };
    // Also can be used in function but very complex to handle generics
    generic_func(auto_type_v.morph_type_variable);
}

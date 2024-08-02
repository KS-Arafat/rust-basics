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

pub fn struct_enum() {
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
}

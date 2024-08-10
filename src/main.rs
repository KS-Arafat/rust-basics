mod _01_variable;
mod _02_function;
mod _03_control_flow_loop;
mod _04_struct_enum;
mod _05_template;
mod _06_error_handling;
mod _07_vector;
mod _08_ownership;
mod _09_lifetime;
mod _10_strings;
fn main() {
    println!("Hello, world!");

    _01_variable::variables();
    _02_function::functions();
    _03_control_flow_loop::control_flow();
    _03_control_flow_loop::loops();
    _04_struct_enum::struct_enum();
    _05_template::generics();
    _06_error_handling::error_handling();
    _07_vector::vectors();
    _08_ownership::ownership();
    _09_lifetime::expl_lifetime();
    _10_strings::strings();
    // Chapter 7 OOP
    // https://tourofrust.com/chapter_7_en.html
}

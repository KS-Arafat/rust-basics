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
mod _11_oop;
mod _12_pointers;
mod _13_smartpointers;
mod _14_io;
mod _15_closure;
mod _16_concurrency;

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
    _11_oop::encapsulation();
    _11_oop::polymorphism();
    _11_oop::dispatch();
    _11_oop::box_stack();
    _12_pointers::pointer();
    _13_smartpointers::smart_p();
    _14_io::cli_input(); // Comment This Line Out for Uninterrupted Execution
    _14_io::file_io();
    _15_closure::anon_fn();
    _16_concurrency::spawningthread();
    _16_concurrency::msg_passing();
    _16_concurrency::mutual_exclusion();
    _16_concurrency::atomic_ref_count();
    _16_concurrency::arc_mutex_chain();
}

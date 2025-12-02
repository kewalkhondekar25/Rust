mod _01_data_type;
mod _02_conditionals;
mod _03_loops;
mod _04_mutability;

fn main() {
    println!("Hello, world!");
    
    println!("-DataTypes-");
    println!("{}", _01_data_type::sum(69, 69));
    println!("{}", _01_data_type::is_even(69));
    println!("{}", _01_data_type::print_string());
    println!("{:?}", _01_data_type::print_vector());
    println!("{:?}", _01_data_type::print_array());

    println!("{}", "Conditionals");
    _02_conditionals::print_conditional(69);

    println!("{}", "loop");
    _03_loops::print_loop();

    println!("{}", "mutability");
    _04_mutability::print_mutable();

}   
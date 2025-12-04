mod _01_data_type;
mod _02_conditionals;
mod _03_loops;
mod _04_mutability;
mod _05_functions;
mod _06_ownership;
mod _07_borrowing;

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

    println!("{}", "functions");
    let ans: u32 = _05_functions::print_sum_function(69, 69);
    println!("{}", ans);

    println!("ownership rules");
    _06_ownership::ownership_rule();
    _06_ownership::ownership_rule_func();

    println!("borowing");
    _07_borrowing::borrowing_func();

    println!("borowing rules");
    _07_borrowing::borrowing_rules();
}   
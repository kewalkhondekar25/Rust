mod _01_data_type;
mod _02_conditionals;
mod _03_loops;

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

    let x: u32 = 3;
    if x % 2 == 0 {
        println!("{}", "2 is even")
    } else {
        println!("{}", "3 is false")
    }
}   
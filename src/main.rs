mod _01_data_type;

fn main() {
    println!("Hello, world!");
    
    println!("-DataTypes-");
    println!("{}", _01_data_type::sum(69, 69));
    println!("{}", _01_data_type::is_even(69));
    println!("{}", _01_data_type::print_string());
    println!("{:?}", _01_data_type::print_vector());
    println!("{:?}", _01_data_type::print_array());
}   
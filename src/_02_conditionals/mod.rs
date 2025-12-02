//conditionals
pub fn print_conditional(num: u32){
    
    fn is_num_even(num: u32) -> bool {
        return num % 2 == 0;
    }

    let is_even: bool = is_num_even(num); 

    if is_even {
        println!("{}", "true")
    } else {
        println!("{}", "false")
    }
}


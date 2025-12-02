//numbers
pub fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

//boolean
pub fn is_even(num: u32) -> bool {
    return  num % 2 == 0;
}

//strings
pub fn print_string() -> String {
    let name: String = String::from("kewal");
    return name;
}

//array
pub fn print_array() -> [u32; 5] {
    let arr: [u32; 5] = [1, 2, 3, 4, 5]; 
    return arr;
}

//vector
pub fn print_vector() -> Vec<u32>{
    let vec: Vec<u32> = vec![69, 69, 69];
    return vec;
}
pub fn borrowing_func(){
    //ownership
    let str = String::from("kewal");
    let len = get_length(&str);
    println!("{} {}", str, len);
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    return len
}
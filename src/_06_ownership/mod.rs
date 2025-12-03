pub fn ownership_rule(){
    let name: String = String::from("kewal");
    let name2: String = name;
    
    //ownership - borrow of moved value: `name`
    // println!("{}", name);

    //valid ownership
    println!("{}", name2);
}

pub fn ownership_rule_func(){
    let name: String = String::from("kewal");
    let string_length = get_length(name);
    
    //borrow of moved value: `name`
    // println!("{}", name);

    println!("{}", string_length);
}

//s becomes new owner
fn get_length(s: String) -> usize {
    return s.len();
}
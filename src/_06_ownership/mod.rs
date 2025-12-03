pub fn ownership_rule(){
    let name: String = String::from("kewal");
    let name2: String = name;
    
    //ownership - borrow of moved value: `name`
    // println!("{}", name);

    //valid ownership
    println!("{}", name2);
}
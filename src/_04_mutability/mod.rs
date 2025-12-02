pub fn print_mutable(){
    //invalid
    // let x: u32 = 69;
    // x = 70;

    //valid
    let mut x: u32 = 69;
    x = 70;
    println!("{}", x);
}
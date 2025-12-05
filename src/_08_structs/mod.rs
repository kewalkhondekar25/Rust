struct Rect {
    height: u32,
    width: u32
}

pub fn print_struct(){
    
    let r = Rect {
        height: 10,
        width: 20
    };

    println!("{}, {}", r.height, r.width)
}
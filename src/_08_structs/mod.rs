struct Rect {
    height: u32,
    width: u32
}

impl Rect {
    //method
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    //static method
    fn get_rect(){
        println!("this is static method")
    }
}

pub fn print_struct(){
    
    let r = Rect {
        height: 10,
        width: 20
    };

    println!("{}, {}", r.height, r.width);
    println!("{}", r.area());
    //call static method
    Rect::get_rect();
}
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

pub fn borrowing_rules(){
   //1. You can have only one mutable reference at a time.
   println!("{}", "rule 1: ");
   
   let mut n = String::from("Rust");

   let a = &mut n;//valid
   //let b = &mut n; - invalid

   println!("{}", a);

   //2. You can have multiple immutable refs, but not mutable + immutable at same time.
   println!("{}", "rule 2: ");

   let mut x = String::from("Hello");

   let ref_one = &x;
   let ref_two = &x;
   //let refThree = &mut x; - invalid

   println!("{}, {}, {}", x, ref_one, ref_two);

   println!("{}", "rule 3: ");
   //3. Mutable reference allowed only when no one else is reading
   
   let mut book = String::from ("mybook");

   let reader = &book;
   println!("{}", reader);

   let editor = &mut book;
   editor.push_str(" v2");
   println!("{}", editor)

}
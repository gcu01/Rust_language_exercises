fn main() {
    println!("Hello, world!");

    let a_main: Box<i32> = Box::new(5);
    let b = a_main;
    //println!("{}", a_main);
    move_a_box(b);
}

fn move_a_box(a: Box<i32>) {
    println!("inside box fn");
}
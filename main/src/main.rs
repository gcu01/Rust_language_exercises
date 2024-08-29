mod structures;
use structures::Rectangle;

fn main() {

    let mut x = Box::new(2);
    let r1 = &*x;
    let r2 = *x;
    let r3 = &r2;
    println!("{}", r1);
    println!("{}", *r3);
let r = ret_str();
println!("{}", r);

let mut a:i32 = 515;
let aa:&i32 = &a;
println!("{}", a);
println!("{}", aa);

let mut tr = dbg!(&30*4);
println!("{}", tr);


let r1 = Rectangle{width:10, height:11,};
println!("area of {:#?} is: {}", r1, r1.area());
let r2:Rectangle = Rectangle{width:40, height:12};
println!("r2 can hold r1 ? ({})", r2.can_hold(&r1));
let r3: Rectangle = Rectangle::new();
println!("{:#?}", r3);

}

fn ret_str() -> String {
    let s = String::from("home");
    s
}
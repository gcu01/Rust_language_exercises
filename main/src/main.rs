fn main() {
    println!("Hello, world!");

    let a_main: Box<i32> = Box::new(5);
    let b = a_main;
    //println!("{}", a_main);
    move_a_box(b);


    let mut x = vec![1,2,3];
    let mut y = &mut x[2];
    *y = 5;
    println!("{} ", *y);
    println!("{} ", *y);
    println!("{} ", *y);
    *y = 6;
    println!("{:?}", x);
    
let mut a = 15;
let mut b = &mut a;
println!("{}", a);

let mut arr:[i32; 5] = [3; 5];
let ref_arr = & arr[2];
//arr[2] = 5;
println!("arr[2] = {}", arr[2]);
let c = 1 + arr[2];
println!("c = {}", c);
//*ref_arr = 7;
println!("*ref_arr = {}", *ref_arr);

let mut xx = 112;
let yy = &xx;
println!("{}, {}", xx, *yy);
let zz = *yy;
xx = 113;
println!("{}, {}", xx, zz);
xx= zz;

let q = vec![1,2,3];
let mut w: Vec<i32> = q;

w[0] = 0;

println!("{}", w[0]);

let aaa: Vec<String> = vec!["mama".to_string(), "tata".to_string()];
let bbb: &str = out_str(&aaa);
println!("{}", bbb);

let mut tr111:i32=0;
try1(&mut tr111);
println!("{}", tr111);

let mut try2:Vec<i32> = vec![1,2,3];
let r1_try2 = &try2[0];
let r2_try2 = &try2[0];
println!("{} & {}", *r1_try2, r2_try2);
let r3_try2: &mut i32 = &mut try2[0];
//println!("{} ", r3_try2);
//println!("{} & {}", r1_try2, r2_try2);

let take_ownership = take_own();
println!("{}", take_ownership);

let mut p = 4;
let pp = &mut p;
*pp = 6;
let ppp = pp;
println!("p={}", *ppp);


let mut s = String::from("hello");
let s_ptr: &str = &s[0..2];
println!("{:?}", s_ptr);
s.push('a');
println!("{:?}", s);

let mut strn = String::from("value");
let strn_ptr = &mut strn;
println!("{}", strn);
//strn_ptr.push_str(" string");

let mut strn_stk = [1,2];
//let strn_ptr_stk = &mut strn_stk[0];
//*strn_ptr_stk=55;
//println!("{:?}", strn_stk);
let strn_ptr_stk = &mut strn_stk;
*strn_ptr_stk=[5,55];
println!("{:?}", strn_stk);

}

fn move_a_box(a: Box<i32>) {
    println!("inside box fn");
}

fn out_str(a: &Vec<String>) -> &str {

    let b = &a[0];
    b
}

fn try1(a: &mut i32){
    *a+=1;
}

fn take_own()->i32 {
    5
}
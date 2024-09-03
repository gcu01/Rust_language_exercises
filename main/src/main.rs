mod enums;

#[derive(Debug)]
enum Test_me{
    T1(usize),
    T2(String),
}

fn main() {
    println!("Hello, world!");

    let s1:Option<i32> = Option::Some(3);
    let s2:Option<usize> = None;

    println!("{:#?} {:#?}", s1, s2);
    let m_s = 3;

     match m_s {
        s1 => println!("s1"),
        s2 => println!("s2"),
    }

    let id:usize = 0;
    let my_st: String = String::from("value");
    let my_st_dummy: String = String::from("value_dummy");
    let s0: Test_me = Test_me::T1(id);
    let s: Test_me = Test_me::T2(my_st);
    let x = match &s {
        
        Test_me::T2(st) => st,
        _ => &my_st_dummy,
        //Test_me::T1(nr) => nr,
    };
    println!("x={:#?}", s);
    println!("x={:#?}", x);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("v={:?}", v);
    v = vec![5,6];
    println!("v={:?}", v);
    println!("v[1]={}", &v[1]);
    v.push(3);
    let v_none: Option<&i32> = v.get(5);
    match v_none {
        None => {println!("no index 5");}
        Some(i) => {println!("at index 4, value: {i}");}
    }

    for i in &v {
        println!("{}", *i);
    }

    for i in &mut v {
        *i +=1;
        println!("{}", *i);
    }

    let mut vv = vec![String::from("hello ")];
    let mut strng_o = &mut vv[0];
    strng_o.push_str("string");
    println!("{}", strng_o);
    

}


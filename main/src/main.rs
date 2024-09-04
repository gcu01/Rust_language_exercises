use std::collections::HashMap;

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

    let mut lst = vec![4,65,2,34,87,6,3,565,0,98,6,71,26, 6, 3, 45];
    lst.sort();

    println!("median of {lst:?} is: {}", &lst[lst.len()/2]);
    
    let mut lst_h: HashMap<&i32, i32> = HashMap::new();
    let mut count: i32 = 0;
    let mut flag: bool = false;
    for id_i32 in &lst {
        flag = false;
        for (k, val) in lst_h.iter_mut() {
            if **k == *id_i32 { 
                dbg!("**k = {}", **k);
                *val += 1; 
                flag = true;
            }
        }
        if flag == false {
            lst_h.insert(id_i32, 1);
        }
        //lst_h.insert(&lst[*i], count);
    }
    println!("{lst:?}");
    println!("{lst_h:?}");

    let mut lst_mode: Vec<i32> = Vec::new();
    let mut occur: i32 = 0;
    for (_, val) in &lst_h{
        if *val > occur {
            occur = *val;
        }
    }
    println!("occur = {}", occur);
    for (k, val) in &lst_h {
        if *val == occur {
            lst_mode.push(**k);
        }
    }
    println!("mode = {lst_mode:?}");
}


use std::io::{self, Read};
use std::fs::File;
#[derive(Debug)]
struct Point<T, TT> {
    x: T,
    y: TT,
}

impl<T, TT> Point<T, TT> {
    fn get_x(&self) -> &T {
        &self.x
    }

    /* fn make_other<L, LL> (&self, other: &Point<L, LL> ) -> Point <&T, &LL> {
        Point {
            x: &self.x,
            y: &other.y,
        }
    }
    */
}
fn main() {

   println!("{:?}", work_file());
    match work_question("aa") {
        Some(i) => {println!("{}", i);}
        None => {println!("None");}
    }

    let x1: Point<String, i32> = Point{x:"mama".to_string(), y: 10};
    let x2: Point<i32, String> = Point {x: 12, y: "tata".to_string()};

    //let mut x3: Point<&str, &str> = Point::make_other(&x1, &x2);
    //println!("{x3:?}");
    println!("{x1:?}");
}

fn work_file() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut f  = File::open("Config1.toml")?.read_to_string(&mut username)?;
    Ok(username)
}

fn work_question(a: &str) -> Option<&str> {
    let mut ty: Option<&str> = None;
    
    match a {
        "a" => {ty = Some(a);}
        _ => {ty = None;}
    }
    
    ty
}
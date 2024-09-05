use std::io::{self, Read};
use std::fs::File;

fn main() {

   println!("{:?}", work_file());
    match work_question("aa") {
        Some(i) => {println!("{}", i);}
        None => {println!("None");}
    }

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
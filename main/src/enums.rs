

pub mod h_maps {
use std::collections::HashMap;
    pub fn maps1(opt: &mut Option<i32>) {
        let mut a:HashMap<String, i32> = HashMap::new();
        a.insert("k1".to_string(), 10);
        a.insert("k2".to_string(), 12);
        *opt = a.get("k1").copied();
    }
    pub fn maps2() -> &'static str{
        let mut a = HashMap::new();
        a.insert("k1", 10);
        a.insert("k2", 11);
        a.insert("k3", 12);

        for (k,v) in &a {
            if *v > 11 {return *k;}
        }

        "0"
    }
}

#[cfg(test)]
mod tests {
    use super::h_maps::*;

    #[test]
    fn test1() {
        let mut a: Option<i32> = None;
        maps1(&mut a);
        match a {
            None => {assert!(false);}
            Some(i) => {assert_eq!(10, i);}
        }
    }

    #[test]
    fn test2(){
        assert_eq!("k3", maps2());
    }

}
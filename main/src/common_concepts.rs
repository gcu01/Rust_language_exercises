const NO_PLAYERS1: i32 = 10;

mod concepts {
    //pub fn vars1
    pub const NO_PLAYERS2: u32 = 11*2;
}

pub mod test_returns {
    pub fn test_rtrn1(a: i32) -> i32 {

        let mut a:i32 = a;
        return a;
    }

    pub fn test_rtrn2(a: i32) -> i32 {

            let mut a:i32 = 1;
            loop {
                break a
            }
    }

    pub fn test_break()-> i32 {
        let mut a = 1;

        'first: loop {
            if a < 5 { a += 1; }
            if a == 5 {
                loop {
                    if true {
                        break 'first;
                    }
                }
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::common_concepts::{NO_PLAYERS1, concepts::NO_PLAYERS2};


    #[test]
    fn test_vars() {
        let mut x = 1;
        println!("{}", x);
        x += 1;
        println!("{}", x);
        assert_eq!(2, x);
    }

    #[test]
    fn test_consts(){
        assert_eq!(10, NO_PLAYERS1);
        assert_eq!(11*2, NO_PLAYERS2);
    }

    #[test]
    fn test_shadowing(){
        let x = 1_usize;
        let x = x + 1_usize;
        assert_eq!(2, x);
    }

    #[test]
    #[deny(overflowing_literals)]
    fn test_overflow(){
        let mut a:u8 = 1;
        //a = 257;
        assert!(1>0);
    }

    #[test]
    fn test_array(){
        let a:[i32;5] = [2;5];
        let m = "I am learning";
        let aa = [m;4];
        assert_eq!("I am learning".to_string(), aa[0]);
        assert_eq!(2, a[2]);
    }

    #[test]
    fn test_while() {
        let mut a = 0;
        let mut b = 0;

        'lbl: loop {
            if a < 3 { a += 1;}
            else { break 'lbl;}
        }

        while b<3 {
            b += 1;
        }

        assert_eq!(a, b);
    }

}
const NO_PLAYERS1: i32 = 10;

mod concepts {
    //pub fn vars1
    pub const NO_PLAYERS2: u32 = 11*2;
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

}
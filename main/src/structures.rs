#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area (&self) -> u32 {
        self.height * self.width
    }
    pub fn can_hold(&self, r2: &Rectangle) -> bool {
        self.width > r2.width && self.height > r2.height 
    }
    pub fn new () -> Self {
        Self{width:10, height:0,}
    }
}
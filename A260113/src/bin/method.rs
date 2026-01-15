#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn rarea(&self) -> u32{
        self.width *self.height/2
    }
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width  > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = [Rectangle { width: 30, height: 50 },Rectangle { width:40 , height:20}];
    for i in 0..2 {
        println!(
            "The area{} of the rectangle is {} square pixels.",
            i,
            rect[i].area()
        );
        println!(
            "The carea{} of the rectangle is {} square pixels.",
            i,
            rect[i].rarea()
        );
        println!("Can rect1 hold rect2?:{}",rect[0].can_hold(&rect[1]));
    }
}
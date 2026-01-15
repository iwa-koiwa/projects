#[derive(Debug)]
struct Rectangle {
    width:u64,
    height:u64,
}

fn main() {
    let rect1 = Rectangle{width:60,height:40};
    println!("rect1 is:{:?}",rect1);
    println!(
        "The area of the Rectangle is {} square pixels.",
        area(&rect1),
    );
}

fn area(dimensions:&Rectangle)->u64{
    dimensions.width * dimensions.height
}
struct rectangle {
    width:u64,
    height:u64,
}

fn main() {
    let area2 = rectangle {
        width:30,
        height:50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(area2.width , area2.height),
    );
}

fn area(width:u64,height:u64)->u64{
    width + height
}
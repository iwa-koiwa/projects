use proconio::input;

fn main() {
    input!{
        s1:u8,
        s2:u8,
        s3:u8,
    }
    let s = [s1,s2,s3];
    let mut output_number = 0;
    for element in s {
        if element == 1 {
            output_number += 1;
        }
    }
    println!("{}",output_number);
}
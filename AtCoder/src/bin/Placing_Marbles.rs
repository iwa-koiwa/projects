use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s:Chars,
    }
    let mut output_number = 0;
    for element in s {
        if element == '1' {
            output_number += 1;
        }
    }
    println!("{}",output_number);
}
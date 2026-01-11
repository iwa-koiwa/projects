use proconio::input;
use proconio::marker::Chars;

fn main(){
    input! {
        s1: Chars,
    };
    let output_number = s1.iter().filter(|&&x| x == '1').count();
    println!(
        "{}" ,output_number
    )
}

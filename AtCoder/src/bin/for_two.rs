use proconio::input;

fn main(){
    input! {
        n:u16,
    }
    let mut a_total =0;
    for i in 0..n{
        if i%2==0{
            input! {
                a:i16
            }
            a_total += a;
        }
        else if i%2 == 1 {
            input! {
                a:i16
            }
            a_total -= a;
        }
    }
    println!("{}",a_total.abs())
}
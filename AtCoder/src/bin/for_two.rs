use proconio::input;

fn main(){
    input! {
        n:i16,
        mut a:[i16;n],
    }
    let mut a_total = 0;
    a.sort();
    for i in 0..n as usize {
        if  i%2 == 0 {
            let a1 = a[i];
            a_total += a1;
        }
        else if i%2==1 {
            let a2 =a[i];
            a_total -= a2;
        }
    }
    println!("{}",a_total.abs());
}
use proconio::input;

fn main() {
    input! {
    n: usize,
    mut a : [i32; n],
    }
    let mut total = 0;
    for i in 0..=n{
        if a[i] % 2 == 1{
            break;
        }
        total = i;
    }
    println!("{}",total);
}
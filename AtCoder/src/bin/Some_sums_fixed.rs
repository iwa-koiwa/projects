use proconio::input;

fn main() {
    input! {
        n:u64,
        a:u64,
        b:u64,
    }
    let mut total = 0;
    for i in 1..=n{
        let mut k = 0;
        let mut temp = i;
        while temp > 0 {
            k += temp % 10;
            temp /= 10;
        }
        if k >= a && k <= b {
            total += i;
        }
    }
    println!("{total}");
}

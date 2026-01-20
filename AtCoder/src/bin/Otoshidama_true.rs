use proconio::input;

fn main() {
    input! {
        n:i64,
        y:i64,
    }
    for i in 0..=n{
        for j in 0..=(n-i){
            let k = n-j-i;

            if 10000*i+5000*j+1000*k == y{
                println!("{i},{j},{k}");
                std::process::exit(0);
            }
        }
    }
    println!("-1 -1 -1");
}
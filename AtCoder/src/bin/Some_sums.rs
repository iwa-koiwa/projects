use proconio::input;

fn main() {
    input! {
        n:u64,
        a:u64,
        b:u64,
    }
    let n_limit:[u64;5] =[1,10,100,1000,10000];
    let mut total = 0;
    for i in 1..=n{
        let mut k = 0;
        for j in 0..i.to_string().chars().count(){
            k += (i / n_limit[j]) % 10;
        }
        if k >= a && k <=b {
            total += i;
        } 
        
    }
    println!("{total}");
}

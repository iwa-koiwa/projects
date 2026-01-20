use::proconio::input;

fn main() {
    input!{
        n:i64,
        mut y:u64,
    }
    let mut ans : [i64;3] = [0,0,0];
    let mut total = 0;
    while total <= n{
        if y%10000 == 0{
            y -= 10000;
            ans[0] += 1;
        }else if y%5000 == 0{
            y -= 5000;
            ans[1] += 1;
        }else if y%1000 == 0{
            y -= 1000;
            ans[2] +=1;
        }else {
            
        }
        total +=1;
        println!("{}",y)
    }
    println!("{:?}",ans);
}
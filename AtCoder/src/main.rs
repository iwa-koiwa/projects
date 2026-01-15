use proconio::input;

fn main(){
    input!{
        a:i64,
        b:i64,
    }
    let ab = a*b%2;
    if ab == 0{
        println!("Odd");
    }else{
        println!("Even");
    }
}

#[derive(Debug)]
enum ipaddrkind {
    V4(u8,u8,u8,u8),
    V6(String)
}

fn main(){
    let home = ipaddrkind::V4(127,0,0,1);

    let loopback = ipaddrkind::V6(String::from("::1"));

    println!("{:?},{:?}",home,loopback);
}

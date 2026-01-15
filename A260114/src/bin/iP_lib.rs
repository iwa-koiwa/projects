use std::{net::{Ipv4Addr, Ipv6Addr}, thread::LocalKey};

#[derive(Debug)]
enum ipaddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

fn main() {
    let localhost_v4 = ipaddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = ipaddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    println!("{:?},{:?}",localhost_v4,localhost_v6);
}
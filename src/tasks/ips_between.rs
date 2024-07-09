// https://www.codewars.com/kata/526989a41034285187000de4/train/rust

use std::net::Ipv4Addr;

pub fn ips_between(start: &str, end: &str) -> u32 {
    let start: u32 = start.parse::<Ipv4Addr>().unwrap().into();
    let end: u32 = end.parse::<Ipv4Addr>().unwrap().into();

    println!("{}", start);
    println!("{}", end);
    end - start
}

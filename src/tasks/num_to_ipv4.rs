use std::net::Ipv4Addr;

pub fn num_to_ipv4(int: u32) -> String {
    // Write your code here
    let ip = Ipv4Addr::from(int.to_be_bytes());

    ip.to_string()
}
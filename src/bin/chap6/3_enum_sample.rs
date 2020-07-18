#[derive(Debug)]
struct Ipv4Addr {
// --snip--
    addr1: u8,
    addr2: u8,
    addr3: u8,
    addr4: u8,
}

#[derive(Debug)]
struct Ipv6Addr {
// --snip--
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let home = IpAddr::V4(Ipv4Addr {
        addr1: 127,
        addr2: 0,
        addr3: 0,
        addr4: 1,
    });
    let loopback = IpAddr::V6(Ipv6Addr {
        address: String::from("::1"),
    });

    println!("home: {:?}, loopback: {:?}", home, loopback);
}

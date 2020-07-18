#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}, loopback: {:?}", home, loopback);


    let home = IpAddr2::V4(127,0,0,1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}

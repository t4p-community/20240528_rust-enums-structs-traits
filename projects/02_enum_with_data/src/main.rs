enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_ip_addr(ip: IpAddrKind) -> String {
    match ip {
        IpAddrKind::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
        IpAddrKind::V6(ip) => format!("{}", ip),
    }
}

fn main() {
    
    let loopback = IpAddrKind::V4(127, 0, 0, 1);
    println!("home: {}", print_ip_addr(loopback));

    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("loopback: {}", print_ip_addr(loopback));

}


fn main() {

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String), 
}

/*
let four = ipAddrKind::V4;
let five = ipAddrKind::V6;

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

} // end main
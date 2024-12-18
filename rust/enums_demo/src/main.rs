enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip_addr(&home);
    print_ip_addr(&loopback);


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_ipv4(){
            let home = IpAddr::V4(127,0,0,1);
            match home {
                IpAddr::V4(a,b,c,d) => {
                    assert_eq!(a,127);
                    assert_eq!(b,0);
                    assert_eq!(c,0);
                    assert_eq!(d,1);

                },
                _ => panic!("Expected IPv4"),
            }
        }

        #[test]
        fn test_ipv6(){
            let loopback = IpAddr::V6(String::from("::1"));
            match loopback {
                IpAddr::V6(addr) => assert_eq!(addr, "::1"),
                _ => panic!("Expected IPv6"),
            }
        }
    }
}

fn print_ip_addr(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a,b,c,d) => println!("IPv4 Address {}.{}.{}.{}", a,b,c,d),
        IpAddr::V6(addr) => println!("IPv6 Address: {}",addr),
    }
}



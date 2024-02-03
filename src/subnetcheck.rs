use std::env;
//use std::net::{Ipv4Addr};
use ipnet::{Ipv4Net, Ipv6Net};

fn main() {
    // Get prefix from the user
    if let Some(prefix) = env::args().nth(1) {
        print!("Supplied prefix is: {}\n", prefix);

        // function result
        let v4_network: bool = valid_ipv4_subnet(&prefix);
        let v6_network: bool = valid_ipv6_subnet(&prefix);

        if v4_network == true {
            // prefix supplied ipv4 
            println!("IPv4 subnet provided.")
        }
        else if v6_network == true {
            println!("IPv6 subnet provided.")

        }
        // Prefix is netiher ipv4 or ipv6
        else {
            println!("Invalid IPv4 / IPv6 subnet provided.")
        }
    }
    // No prefix supplied branch
    else {
        println!("A valid IPv4 or Ipv6 prefix in CIDR notation must be supplied!\nExample: \'subnetcheck 10.0.0.0/24\'")
    }
}

fn valid_ipv4_subnet(prefix:&str)-> bool{
    // pass in the user supplied prefix, check if it is a validate IPv4 subnet
    prefix.parse::<Ipv4Net>().is_ok()
}

fn valid_ipv6_subnet(prefix:&str)-> bool{
    // pass in the user supplied prefix, check if it is a validate IPv6 subnet
    prefix.parse::<Ipv6Net>().is_ok()
}

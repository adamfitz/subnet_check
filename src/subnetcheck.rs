use std::env;
use ipnet::{Ipv4Net, Ipv6Net, Ipv4AddrRange, IpNet};
//use dns_lookup::lookup_addr;

fn main() {
    // Get prefix from the user
    if let Some(prefix) = env::args().nth(1) {
        print!("Supplied input is: {}\n", prefix);

        // validate user input
        let v4_network: bool = valid_ipv4_subnet(&prefix);
        let v6_network: bool = valid_ipv6_subnet(&prefix);

        if v4_network == true {
            // prefix supplied ipv4 
            println!("IPv4 subnet provided.");
            let ipv4_host_ips = ipv4_hosts(&prefix);
            println!("{:?}", ipv4_host_ips);


            // get the number of hosts in the subnet.

        }
        else if v6_network == true {
            println!("IPv6 subnet provided.")

        }
        // Prefix is netiher ipv4 or ipv6
        else {
            println!("Invalid IPv4 / IPv6 subnet provided.  Must be a valid CIDR block slash notation")
        }
    }
    // No prefix supplied branch
    else {
        println!("A valid IPv4 or Ipv6 prefix in CIDR notation must be supplied!\nExample: \'subnetcheck 10.0.0.0/24\'")
    }
}

// function checks if a valid IPv4 subnet
fn valid_ipv4_subnet(prefix:&str)-> bool{
    prefix.parse::<Ipv4Net>().is_ok()
}

// function checks if a valid IPv6 subnet
fn valid_ipv6_subnet(prefix:&str)-> bool{
    prefix.parse::<Ipv6Net>().is_ok()
}

// function returns the number of hosts in an Ipv4 subnet
fn ipv4_hosts(prefix:&str){
    //let hosts = Ipv4Net(prefix)

    // create a network object from the given prefix?
    let net: IpNet = prefix.parse().unwrap();

    println!("{}", net);

    // Yay this works! :)
    for i in net.hosts(){
        println!("{}", i);
    }
    //let num_hosts = Ipv4AddrRange::new(prefix.parse().unwrap());
}

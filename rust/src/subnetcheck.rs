use std::{env, fs};
use ipnet::{Ipv4Net, Ipv6Net, IpNet};
use dns_lookup::lookup_addr;


fn main() {
    // Get prefix from the user
    if let Some(prefix) = env::args().nth(1) {
        print!("Supplied input is: {}\n", prefix);

        // validate user input
        let v4_network: bool = valid_ipv4_subnet(&prefix);
        let v6_network: bool = valid_ipv6_subnet(&prefix);

        // prefix supplied is IPv4
        if v4_network == true {

            // get the number of hosts in the subnet.
            let ipv4_host_ips = ipv4_hosts(&prefix);
            println!("IPv4 subnet provided is valid: {:?}", ipv4_host_ips);

            // iterate all hosts
            for address in ipv4_host_ips.hosts(){
                // attempt reverse lookup and ignore the error (no PTR exists)
                //TODO:  Catch the specific lookup error otherwise panic
                match lookup_addr(&address) {
                    Ok(ptr) => {
                        println!("{} - {}", address, ptr);
                    },
                    Err(_) => {
                        continue;
                    }
                }
            }
        }

        else if v6_network == true {
            // repeat for ipv6
            let ipv6_host_ips = ipv6_hosts(&prefix);
            println!("IPv6 subnet provided is valid: {:?}", ipv6_host_ips);
            println!("IPv6 hosts object: {:?}", ipv6_host_ips.hosts());

            let resolver_ip:() = get_dns_server_linux();

            println!("IPv6 subnet provided.")
        }

        // Prefix is netiher ipv4 or ipv6
        else {
            println!("Invalid IPv4 / IPv6 subnet provided.  Supplied prefix MUST be valid CIDR Notation.")
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
fn ipv4_hosts(prefix:&str) -> IpNet{

    // create a network object from the given prefix
    let ipv4_net: IpNet = prefix.parse().unwrap();

    return ipv4_net
}

// function returns the number of hosts in an Ipv6 subnet
fn ipv6_hosts(prefix:&str) -> Ipv6Net{

    // create a network object from the given prefix
    let ipv6_net: Ipv6Net = prefix.parse().unwrap();

    return ipv6_net
}

// read /etc/resolv.conf for the resolver IP
fn get_dns_server_linux() {
    let contents = fs::read_to_string("/etc/resolv.conf").expect("Failed to open resolv.conf");
    let config = resolv_conf::Config::parse(&contents).unwrap();

    // return name servers from the local machine
    for nameserver in &config.nameservers {
        println!("{}", nameserver.to_string());
    }
}
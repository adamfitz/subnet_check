use std::env;
use std::net::SocketAddr;
use std::process::exit;
use ipnet::{Ipv4Net, Ipv6Net, IpNet};
use dns_lookup::lookup_addr;
use indicatif::{ProgressBar, ProgressStyle};



fn main() {
    // Get prefix from the user
    if let Some(prefix) = env::args().nth(1) {
        //print!("Valid Input provided: {}\n", prefix);

        // validate user input
        let v4_network: bool = valid_ipv4_subnet(&prefix);
        let v6_network: bool = valid_ipv6_subnet(&prefix);

        // prefix supplied is IPv4
        if v4_network == true {

            // get the number of hosts in the subnet.
            let ipv4_host_ips = ipv4_hosts(&prefix);
            // println!("IPv4 start address: {}\nIPv4 End Address: {}", ipv4_host_ips.network(), ipv4_host_ips.broadcast());
            println!("Attempting reverse DNS lookup for the input {}", prefix);

            // implement progress bar
            let ipv4_total_items = ipv4_host_ips.hosts().count() as u64;
            println!("Number of IPv4 hosts: {}\n", ipv4_total_items.to_string());
            let ipv4_progress_bar = ProgressBar::new(ipv4_total_items);
            ipv4_progress_bar.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .expect("Failed to create progress style"));

            // iterate all hosts
            for address in ipv4_host_ips.hosts(){
                // attempt reverse lookup and ignore the error (no PTR exists)
                match lookup_addr(&address) {
                    Ok(ptr) => {
                        // increment the bar
                        ipv4_progress_bar.inc(1);
                        println!("{} - {}", address, ptr);
                    },
                    Err(_) => {
                        // increment the bar
                        ipv4_progress_bar.inc(1);
                        continue;
                    }
                }
            }
            // finish progress
            ipv4_progress_bar.finish();
        }

        else if v6_network == true {
            // repeat for ipv6
            let ipv6_host_ips = ipv6_hosts(&prefix);
            //println!("IPv6 start address: {}\nIPv6 End Address: {}", ipv6_host_ips.network(), ipv6_host_ips.broadcast());
            validate_ipv6_prefix_size(&prefix);
            println!("Attempting reverse DNS lookup for the input {}", prefix);

            //implement progress bar
            let ipv6_total_items = ipv6_host_ips.hosts().count();
            println!("Number of IPv6 hosts: {}\n", ipv6_total_items.to_string());
            let ipv6_progress_bar = ProgressBar::new((ipv6_total_items as u128).try_into().unwrap());
            ipv6_progress_bar.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .expect("Failed to create progress style"));


            for ipv6_address in ipv6_host_ips.hosts() {
                // lookup address function does not work with Ipv6 address
                // convert to socket address and then attempt the PTR lookup
                let socket_addr = SocketAddr::from((ipv6_address, 0));
                match lookup_addr(&socket_addr.ip()) {
                    Ok(ipv6_ptr) => {
                        println!("{} - {}", ipv6_address, ipv6_ptr);
                    }
                    Err(_) => {
                        continue;
                    }
                }
                //increment for each iteration
                ipv6_progress_bar.inc(1);
            }
            //finish progress
            ipv6_progress_bar.finish();
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

    ipv4_net
}

// function returns the number of hosts in an Ipv6 subnet
fn ipv6_hosts(prefix:&str) -> Ipv6Net{

    // create a network object from the given prefix
    let ipv6_net: Ipv6Net = prefix.parse().unwrap();

    ipv6_net
}

// function checks if the provided ipv6 prefix is /64 or larger and if not exits
fn validate_ipv6_prefix_size(prefix: &str) {
    if let Some(prefix_len) = prefix.split('/').nth(1) {
        if let Ok(prefix_num) = prefix_len.parse::<u8>() {
            if prefix_num <= 64 {
                println!("The provided IPv6 prefix MUST be great than /64, Eg: from /65 to /128.");
                exit(0);
            }
        }
    }
}

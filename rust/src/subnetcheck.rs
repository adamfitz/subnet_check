use std::{env, time};
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

        let mut result: Vec<String> = vec![];

        // prefix supplied is IPv4
        if v4_network == true {
            // get the number of hosts in the subnet.
            let ipv4_host_ips = ipv4_hosts(&prefix);

            //print out info to the user
            println!("\nAddress family:\t\tIPv4");
            println!("Address block:\t\t{}", prefix);
            println!("Subnet mask:\t\t{:?}", ipv4_host_ips.netmask());
            println!("Total addresses:\t{}\n", ipv4_host_ips.hosts().count());

            // implement progress bar
            let ipv4_total_items = ipv4_host_ips.hosts().count() as u64;
            let ipv4_progress_bar = ProgressBar::new(ipv4_total_items);
            ipv4_progress_bar.set_style(ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .expect("Failed to create progress style")
                .progress_chars("#>-"));

            // start lookup timer
            let start = time::Instant::now();

            // iterate all hosts
            for address in ipv4_host_ips.hosts(){
                // attempt reverse lookup and ignore the error (no PTR exists)
                match lookup_addr(&address) {
                    Ok(ptr) => {
                        // increment the bar
                        ipv4_progress_bar.inc(1);
                        // concatenate the owned var address, cast to string and join with the reference to the record
                        let output: String = address.to_string() + "\t" + &ptr;
                        result.push(output);
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
            // display the elapsed time
            println!("\n\nReverse lookup operation completed in: {:?}", start.elapsed());
            // Display the number of records returned
            println!("Total number of DNS records found: {}", result.len());
            // If no records are returned dont print anything
            if result.len() > 0 {
                // iterate over the result vector printing all results
                println!("\nList of DNS Records found in IPv4 subnet: {}", &prefix);
                for address in result.iter() {
                    println!("{}", address);
                }
            }
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
                .expect("Failed to create progress style")
                .progress_chars("#>-"));

            // start lookup timer
            let start = time::Instant::now();

            for ipv6_address in ipv6_host_ips.hosts() {
                // lookup address function does not work with Ipv6 address
                // convert to socket address and then attempt the PTR lookup
                let socket_addr = SocketAddr::from((ipv6_address, 0));
                match lookup_addr(&socket_addr.ip()) {
                    Ok(ipv6_ptr) => {
                        // increment the bar
                        ipv6_progress_bar.inc(1);
                        // concatenate the owned var address, cast to string and join with the reference to the record
                        let output: String = ipv6_address.to_string() + "\t" + &ipv6_ptr;
                        result.push(output);
                    }
                    Err(_) => {
                        ipv6_progress_bar.inc(1);
                        continue;
                    }
                }
            }
            //finish progress
            ipv6_progress_bar.finish();
            // display the elapsed time
            println!("Reverse lookup operation completed in: {:?}", start.elapsed());
            // iterate over the result vector printing all results
            println!("DNS reverse lookup results for IPv6 subnet: {}", &prefix);
            for address in result.iter() {
                println!("{}", address);
            }
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

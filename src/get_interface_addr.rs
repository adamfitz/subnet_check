use local_ip_address::list_afinet_netifas;


fn main() {
    // return ips form the local machine
    list_interfaces();
}

// list interface addresses

fn list_interfaces(){
    let network_interfaces = list_afinet_netifas().unwrap();

    for (name, ip) in network_interfaces.iter() {
        println!("{}:\t{:?}", name, ip);
    }
}
use std::fs;

fn main() {
    get_dns_server_linux()

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
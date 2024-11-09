package main

import (
	"fmt"
	"net"
	"os"
)


func host_ips(ipNet *net.IPNet) (net.IP, net.IP) {
	// Calculate and return the first and last host IPs in the provided prefix
	ip := ipNet.IP
	mask := ipNet.Mask

	//calculaute first host ip
	first_host := make(net.IP, len(ip))
	copy(first_host, ip)
	for i := range first_host {
		first_host[i] &= mask[i]
	}
	first_host[len(first_host)-1]++

	// calculate last host ip
	last_host := make(net.IP, len(ip))
	copy(last_host, ip)
	for i := range last_host {
		last_host[i] |= ^mask[i]
	}

	return first_host, last_host
}


func check_address(prefix string) *net.IPNet {
	// Check that the IPv4 or IPv6 prefix is a valid prefix, print out the protocol and start/end host addresses.
	_, ipNet, err := net.ParseCIDR(prefix)
	if err != nil {
		fmt.Printf("Invalid prefix provided: %s\n", prefix)
		return nil
	}

	ip := ipNet.IP
	var prefix_type string
	if ip.To4() != nil {
		prefix_type = "IPv4"
	} else {
		prefix_type = "IPv6"
	}

	first_host, last_host := host_ips(ipNet)

	fmt.Printf("\nValid %s prefix provided: %s\n", prefix_type, prefix)
	fmt.Printf("Protocol: %s\n", prefix_type)
	fmt.Printf("Start Address: %s\n", first_host)
	fmt.Printf("End Address: %s\n\n", last_host)

	return ipNet
}


func reverse_lookup(ip net.IP, last_ip net.IP) {
	// Perform a reverse DNS lookup for each IPv4 or IPv6 address in the provided address block.

	increment_ip := func(ip net.IP) net.IP {
		// increments and returns the next IP for the provided address block
		inc_ip := make(net.IP, len(ip))
		copy(inc_ip, ip)
		for element := len(inc_ip) -1; element >= 0; element-- {
			inc_ip[element]++
			if inc_ip[element] > 0 {
				break
			}
		}
		return inc_ip
	}

		// perform the reverse DNS lookup
		for ; !ip.Equal(last_ip); ip = increment_ip(ip) {
			name, err := net.LookupAddr(ip.String())
			if err == nil {
				fmt.Printf("%s - %s\n", ip, name[0])
			}
		}
	}


func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: subnet_check <IPv4 / IPv6 prefix>")
		os.Exit(1)
	}

	prefix := os.Args[1]
	ipNet := check_address(prefix)
	if ipNet != nil {
		first_host, last_host := host_ips(ipNet)
		reverse_lookup(first_host, last_host)
	}

}
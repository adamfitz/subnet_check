## subnet_check



### bash/subnectCheckv1.sh
#### Description:

Iterate though all IPv4 addresses in a /24 and output any A records found within the given subnet.

### python/subnet_check.py
#### Description:

Takes an IPv4 or IPv6 address block in CIDR notation, performs a DNS lookup for all addresses in the provided prefix and returns all records found.

### rust/src/subnetcheck.rs
#### Description:

Takes an IPv4 or IPv6 (/65 or greater only) prefix, performs a DNS lookup for all addresses in the provided prefix and returns all records found.

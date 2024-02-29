#include <stdio.h>
#include <libcidr.h>

// string length including mask - IPv4 eg: 111.111.111.111/32
#define IPV4_LEN 18

int main() {

    // declare the str array
    char ipv4_string[IPV4_LEN];

    printf("Enter an IPv4 CIDR address block: ");
    fgets(ipv4_string, sizeof(ipv4_string), stdin);

    // check for error or null value
    if (ipv4_string[0] == '\n') {
        printf("Error: No input provided.\n");
        return 1;
    }
    
    printf("IPv4 CIDR provided: %s\n", ipv4_string);

    return 0;

}


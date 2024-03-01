#include <stdio.h>
#include <libcidr.h>
#include <string.h>
#include <arpa/inet.h>


// string length including mask - IPv4 eg: 111.111.111.111/32
#define IPV4_LEN 18

int main() {

    // declare the str array
    char ipv4_string[IPV4_LEN];

    printf("Enter an IPv4 CIDR address block: ");
    fgets(ipv4_string, sizeof(ipv4_string), stdin);

    // check for error or null value
    if (ipv4_string[0] == '\n'){
        printf("Error: No input provided.\n");
        return 1;
    }

    // Remove trailing newline if present
    size_t len = strlen(ipv4_string);
    if (len > 0 && ipv4_string[len-1] == '\n')
        ipv4_string[len-1] = '\0';

    // Parse CIDR block
    struct cidr_addr *cidr_block = cidr_from_str(ipv4_string);
    if (cidr_block == NULL) {
        printf("Error: Invalid CIDR block\n");
        return 1;
    }

    printf("CIDR block %s is valid\n", ipv4_string);

    // free after use CIDR block
    cidr_free(cidr_block);
    
    printf("IPv4 CIDR provided: %s\n", ipv4_string);

    return 0;

}


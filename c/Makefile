CC=gcc
CFLAGS=-Wall -Wextra -pedantic -std=c99
LDFLAGS=-lcidr

subnet_check: subnet_check.o
	$(CC) $(CFLAGS) -o subnet_check subnet_check.o $(LDFLAGS)

subnet_check.o: subnet_check.c
	$(CC) $(CFLAGS) -c subnet_check.c

clean:
	rm -f subnet_check subnet_check.o

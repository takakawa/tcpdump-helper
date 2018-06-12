# TCPDUMP-HELPER
A tool to generate tcpdump commands.
# help
```
tcpdump-helper 1.0
Gaochuan <takakawa@163.com>
Does something funny

USAGE:
    tcpdump-helper [FLAGS] [OPTIONS]

FLAGS:
    -A, --asic       print the packet by asic
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --dev <dev>          set the dev to filter packets
    -X, --method <method>    set the method to filter packets
    -p, --port <port>        set the port to filter packets
    -u, --url <URL>          set the url to filter packets
```
# example
```
~ ./target/debug/tcpdump-helper -A -i em1 -X GET -p 12345 -u /users
tcpdump -i em1 -Ann ' port 12345 and  tcp[((tcp[12:1] & 0xf0) >> 2):4]=0x47455420 and tcp[((tcp[12:1] & 0xf0) >> 2)+5:4]=0x2F757365'
```

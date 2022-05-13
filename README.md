# zack
Plot numeric CSV (-ish) streams in real-time

## Installation

### Using the Rust toolchain

```
git clone https://github.com/wullewutz/zack
cd zack
cargo install --path .
```

## Usage
```
USAGE:
    zack [OPTIONS]

OPTIONS:
    -b, --buffer <buf_length>    How many points of each channels should be displayed before
                                 dropping the oldest [default: 10000]
    -h, --help                   Print help information
    -t, --tcp <host:port>        Tcp socket from where to read csv stream
    -V, --version                Print version information
```

## Examples

### Pipe csv data via stdin into zack

To generate random CSV test data from /dev/urandom, use hexdump and some format strings.
```
hexdump -e '4/1 "%d,"' -e'"\n"' /dev/urandom 
-94,74,-76,58,
-69,-1,-18,-103,
60,-56,-18,27,
-99,-56,108,48,
...
```
Then just pipe the four channels into zack and enjoy the show. 

```
hexdump -e '4/1 "%d,"' -e'"\n"' /dev/urandom | zack --buffer 100
```
Change the `--buffer` length if required.

### Use tcp socket streaming

In this example, the random numbers of the previous example are streamed using
netcat on TCP port 5555.
```
hexdump -e '4/1 "%d,"' -e'"\n"' /dev/urandom | netcat -l -p 5555
```
Connect `zack` to the stream using the `-t` option:
```
zack -t localhost:5555
```

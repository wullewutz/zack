# zack
Plot CSV (-ish) streams in realtime

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
    zack.exe [OPTIONS]

OPTIONS:
    -b, --buffer <buf_length>    How many points of each channels should be displayed before dropping
                                 the oldest [default: 10000]
    -h, --help                   Print help information
    -V, --version                Print version information
```

## Examples

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


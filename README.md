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
Usage: zack [OPTIONS]

Options:
  -b, --buffer <buf_length>  How many points of each channels should
                             be displayed before dropping the oldest.
                             Has to be a power of 2 [default: 16384]
  -t, --tcp <host:port>      Tcp socket from where to read csv stream
  -n, --names <chan_names>   Comma/semicolon/space separated list of channel names
                             Enclose list in quotes if using space or semicolon for separation!
                             Example:     --names first,second,third
                             Equivalent:  --names "first second;third"
  -h, --help                 Print help
  -V, --version              Print version
```

Press w to toggle between separate plots for each channel.
Press space to pause/continue plotting.

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
hexdump -e '4/1 "%d,"' -e'"\n"' /dev/urandom | zack --buffer 128
```
Change the `--buffer` length if required (value needs to be a power of two).

### Use tcp socket streaming

In this example, the random numbers of the previous example are streamed using
netcat on TCP port 5555 from the server:
```
hexdump -e '4/1 "%d,"' -e'"\n"' /dev/urandom | netcat -l -p 5555
```
On the client side, connect `zack` to the stream using the `-t` option:
```
zack -t localhost:5555
```

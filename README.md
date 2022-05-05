# zack
Plot CSV (-ish) streams in realtime

```
USAGE:
    zack.exe [OPTIONS]

OPTIONS:
    -b, --buffer <buf_length>    How many point of each channels should be displayed before dropping
                                 the oldest [default: 10000]
    -h, --help                   Print help information
    -V, --version                Print version information
```

## Examples

To generate random CSV test data from /dev/urandom, use hexdump and a format string.
```
hexdump -e '4/4 "%d,"' -e'"\n"' /dev/urandom 
-1617300816,-672308164,-1556257037,-1823473178,
-670983474,928040320,-1026090360,725147462,
1924729641,-1599148458,936062692,882386033,
765873964,520878765,-1450881801,-1590079812,
579302332,1153731875,-734069802,454871944,
...
```
Then just pipe the four channels into zack and enjoy the show. 

```
hexdump -e '4/4 "%d,"' -e'"\n"' /dev/urandom | zack --buffer 100
```
Change the `--buffer` length if required


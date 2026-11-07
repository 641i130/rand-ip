# rand-ip

Pick a random IPv4 address inside a given CIDR network, written in Rust.

## Usage

```
rand-ip [CIDR] [OPTIONS]
```

- **CIDR** — a v4 network, e.g. `192.168.1.0/24` (defaults to `35.111.67.88/30`)
- **`-n`, `--count N`** — print N random IPs, one per line (`0` prints network info only)
- **`-h`, `--help`** — show help

## Examples

```console
$ rand-ip 10.0.0.0/24
Generated IP: 10.0.0.173
Broadcast: 10.0.0.255

$ rand-ip 192.168.1.0/24 --count 5
192.168.1.90
192.168.1.204
192.168.1.37
192.168.1.241
192.168.1.118

$ rand-ip 35.111.67.88/30 --count 0
Network: 35.111.67.88/30
Broadcast: 35.111.67.91
```

## Building

```
cargo build --release
```

The binary lands in `target/release/rand-ip`.

## License

GPL-3.0 (see [LICENSE](LICENSE)).

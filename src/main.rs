use ipnet::Ipv4Net;
use iprange::IpNet;
use rand::{self, Rng};
use std::net::Ipv4Addr;

/// A single random IP inside `net`.
///
/// The network's fixed bits are kept and the remaining host bits are
/// randomised. (Bit-folding trick courtesy of zwerdlds.)
fn random_ip_in(net: &Ipv4Net, rng: &mut impl Rng) -> Ipv4Addr {
    net.prefix_bits()
        .enumerate()
        .map(|(i, b)| (i as u8, b))
        .chain((net.prefix_len()..32).map(|i| (i, rng.gen::<bool>())))
        .filter(|(_, b)| *b)
        .map(|(i, _)| 2_u32.pow((31 - i) as u32))
        .fold(0_u32, |prev, i| prev + i)
        .into()
}

/// The broadcast address of `net` (all host bits set).
fn broadcast(net: &Ipv4Net) -> Ipv4Addr {
    net.prefix_bits()
        .enumerate()
        .map(|(i, b)| (i as u8, b))
        .chain((net.prefix_len()..32).map(|i| (i, true)))
        .filter(|(_, b)| *b)
        .map(|(i, _)| 2_u32.pow((31 - i) as u32))
        .fold(0_u32, |prev, i| prev + i)
        .into()
}

const USAGE: &str = "\
rand-ip — pick a random IP address inside an IPv4 CIDR network

Usage:
  rand-ip [CIDR] [OPTIONS]

Arguments:
  CIDR          A v4 network, e.g. 192.168.1.0/24
                (default: 35.111.67.88/30)

Options:
  -n, --count N   Print N random IPs, one per line (0 = network info only)
  -h, --help      Show this help and exit

Examples:
  rand-ip 10.0.0.0/24
  rand-ip 192.168.1.0/24 --count 5
  rand-ip 35.111.67.88/30 --count 0";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_cidr = "35.111.67.88/30";
    let mut cidr = default_cidr.to_string();
    let mut count: usize = 1;

    let mut it = args.iter().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return;
            }
            "-n" | "--count" => {
                let value = match it.next() {
                    Some(v) => v,
                    None => {
                        eprintln!("error: --count expects a number");
                        std::process::exit(2);
                    }
                };
                match value.parse() {
                    Ok(n) => count = n,
                    Err(_) => {
                        eprintln!("error: invalid count '{value}'");
                        std::process::exit(2);
                    }
                }
            }
            other => {
                if other.starts_with('-') && other != "-" {
                    eprintln!("error: unknown option '{other}'");
                    eprintln!();
                    eprintln!("{USAGE}");
                    std::process::exit(2);
                } else {
                    cidr = other.to_string();
                }
            }
        }
    }

    let a_net: Ipv4Net = match cidr.parse() {
        Ok(net) => net,
        Err(_) => {
            eprintln!(
                "error: '{cidr}' is not a valid IPv4 CIDR (try e.g. 192.168.1.0/24)"
            );
            std::process::exit(1);
        }
    };

    let brd = broadcast(&a_net);
    if count == 0 {
        println!("Network: {a_net}");
        println!("Broadcast: {brd}");
        return;
    }

    let mut rng = rand::thread_rng();
    if count == 1 {
        let ip = random_ip_in(&a_net, &mut rng);
        println!("Generated IP: {ip}");
        println!("Broadcast: {brd}");
    } else {
        for _ in 0..count {
            println!("{}", random_ip_in(&a_net, &mut rng));
        }
    }
}

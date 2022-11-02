use std::net::Ipv4Addr;
use rand::seq::IteratorRandom;
use regex::Regex;

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn subnet(subnet_mask: &String) -> Result<Vec<Ipv4Addr>,Box<dyn std::error::Error>> {
    // GENERATE VECTOR OF IP ADDRESSES IN SUBNET MASK!!!
    let seperator = Regex::new(r"([./])").expect("Invalid regex");
    let mut cidr:Vec<u8> = Vec::new();
    for c in split_keep(&seperator, subnet_mask) {
        if c != "." && c != "/" {
            cidr.push(c.parse::<u8>().expect("IP address invalid"));
        }
    }

    println!("{:?}",cidr);
    return Ok(vec![Ipv4Addr::new(127, 0, 0, 1),Ipv4Addr::new(127, 0, 0, 1),Ipv4Addr::new(127, 0, 0, 1)])
}

fn main() {
    println!("{:?}",subnet(&"172.16.0.0/16".to_string()))
    //let mut rng = rand::thread_rng();
    //println!("{}", ip_range.iter().choose(&mut rng).unwrap());
}

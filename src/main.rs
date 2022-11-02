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
    let mut cidr:Vec<u32> = Vec::new();
    for c in split_keep(&seperator, subnet_mask) {
        if c != "." && c != "/" {
            cidr.push(c.parse::<u32>().expect("IP address invalid"));
        }
    }
    if cidr.len() != 5 {
        return Err("Please use the format '1.1.1.1/8' for example.".into())
    }

    let ipnum = (cidr[0] << 24) | (cidr[1] << 16) | (cidr[2] << 8) | (cidr[3]);
    let mask:u32 = 0xffffffff << (32 - cidr[4]);
    
    let ipstart:u32 = ipnum & mask;
    //let ipstart:Ipv4Addr = Ipv4Addr::from((ipnum & mask).to_be_bytes());
    //let ipend:Ipv4Addr = Ipv4Addr::from((ipnum | (mask ^ 0xffffffff)).to_be_bytes());
    let ipend:u32 = ipnum | (mask ^ 0xffffffff);
    
    let mut out:Vec<Ipv4Addr> = Vec::new();
    for val in 0..ipend-ipstart {
        out.push(Ipv4Addr::from((val+ipnum).to_be_bytes()).to_owned());
    }

    return Ok(out)
}

fn main() {
    let list:Vec<Ipv4Addr> = subnet(&"172.16.0.0/16".to_string()).unwrap();
    println!("Total IP addresses : {:?}",list.len());
    //let mut rng = rand::thread_rng();
    //println!("{}", ip_range.iter().choose(&mut rng).unwrap());
}

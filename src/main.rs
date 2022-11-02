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
    println!("{:?}",cidr);

    let ipnum = (cidr[0] << 24) | (cidr[1] << 16) | (cidr[2] << 8) | (cidr[3]);
    let mask:u32 = 0xffffffff << (32 - cidr[4]);
    
    let ipstart:u32 = ipnum & mask;
    //let ipstart:Ipv4Addr = Ipv4Addr::from((ipnum & mask).to_be_bytes());
    //let ipend:Ipv4Addr = Ipv4Addr::from((ipnum | (mask ^ 0xffffffff)).to_be_bytes());
    let ipend:u32 = ipnum | (mask ^ 0xffffffff);

    for val in 0..ipend-ipstart {
        println!("{:?}",Ipv4Addr::from(val.to_be_bytes()));
    }

    println!("Addresses : \n{:?}\n{:?}",ipstart,ipend); 

    return Ok(vec![Ipv4Addr::new(127, 0, 0, 1),Ipv4Addr::new(127, 0, 0, 1),Ipv4Addr::new(127, 0, 0, 1)])
}

fn main() {
    println!("{:?}",subnet(&"172.16.0.0/16".to_string()))
    //let mut rng = rand::thread_rng();
    //println!("{}", ip_range.iter().choose(&mut rng).unwrap());
}

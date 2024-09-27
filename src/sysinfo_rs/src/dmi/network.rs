use reqwest::blocking;
use get_if_addrs::get_if_addrs;
use std::net::IpAddr;
use sysinfo::Networks;

pub fn get_public_ip() -> Result<String, reqwest::Error> {
    let server = "https://api.ipify.org";
    let response = blocking::get(server)?.text()?;

    Ok(response)
}

pub fn get_local_ip() -> (String,String) {
    let if_addrs = get_if_addrs().unwrap();
    let mut output1 = String::new();
    let mut output2 = String::new();

    for iface in if_addrs {
        if let IpAddr::V4(ipv4) = iface.addr.ip() {
            if !ipv4.is_loopback() {
                output1 = format!("{}", iface.name);
                output2 = format!("{}",ipv4);
                break;
            }
        }
    }

    if output1.is_empty() {
        output1 = "No Found".to_string();
        output2 = "".to_string(); 
    }

    return (output1,output2)
}

pub fn get_speed() -> (String,f64,f64) {
    let if_addrs = get_if_addrs().unwrap();
    let networks = Networks::new_with_refreshed_list();
    let mut networkcard = String::new(); 
    let mut received_mb:f64 = 0.0;
    let mut transmitted_mb:f64 = 0.0;

    for iface in if_addrs {
        if let IpAddr::V4(ipv4) = iface.addr.ip(){
            if !ipv4.is_loopback(){
                networks.into_iter().for_each(|(iface, data)| {
                    networkcard = iface.to_string();
                    received_mb = (data.total_received() / 1024 / 1024) as f64;
                    transmitted_mb = (data.total_transmitted() / 1024 / 1024) as f64;
                });
            }
        }
    }

    if networkcard.is_empty() {
        networkcard = "No Found".to_string();
        received_mb = 0.0 ;
        transmitted_mb = 0.0 ;
    }

    return(networkcard,transmitted_mb, received_mb)
}

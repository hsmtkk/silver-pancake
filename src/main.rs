mod env;

use std::collections::HashMap;

const UPDATE_URL: &str = "https://dynupdate.no-ip.com/nic/update";

fn main() {
    let username = env::mandatory_string("USERNAME");
    let password = env::mandatory_string("PASSWORD");
    let hostname = env::mandatory_string("HOSTNAME");
    let ip_address = env::mandatory_string("IP_ADDRESS");

    let mut query = HashMap::new();
    query.insert("hostname", hostname);
    query.insert("myip", ip_address);

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(UPDATE_URL)
        .basic_auth(username, Some(password))
        .query(&query)
        .send()
        .expect("send HTTP request");
    if resp.status().is_success() {
        println!("OK");
    } else {
        println!("Error");
    }
    println!("{}", resp.text().expect("response as text"));
}

//domain list
 pub let mut domains: Vec<Domain> = Vec::new();
#[derive(Debug)]
pub struct Domain {
    name: String,
    protocol: String,
    address: String,
    port: u16,
}
pub fn domain_test() -> Domain {
    let google = Domain {
        name: "Google".to_string(),
        protocol: "https://".to_string(),
        address: "google.com".to_string(),
        port: 443,
    };

    return google;
}

pub fn add_domain(name: String, protocol: String, address: String, port: u16) -> Domain {
    Domain { name, protocol, address, port }

}
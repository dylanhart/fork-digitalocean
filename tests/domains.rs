extern crate digitalocean;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
extern crate url;
extern crate url_serde;

mod utils;

use serde_json::Value;
use std::net::IpAddr;
use std::str::FromStr;
use std::env;

use digitalocean::api::Domains;
use digitalocean::{Retrievable, DigitalOcean};
use digitalocean::request::Request;
use digitalocean::values::Domain;
use digitalocean::action::{Get, List, Create, Delete};

use utils::before;

#[test]
fn list_produces_correct_request() {
    before();

    let correct_url = "https://api.digitalocean.com/v2/domains";

    let req: Request<List, Vec<Domain>> = Domains::list();
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn create_produces_correct_request() {
    before();

    let domain = "example.com";
    let ip_address = IpAddr::from_str("192.168.0.1").unwrap();
    let correct_url = "https://api.digitalocean.com/v2/domains";

    let req: Request<Create, Domain> = Domains::create(domain, ip_address);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, json!({
        "name": domain,
        "ip_address": ip_address,
    }));
}

#[test]
fn get_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}", domain);

    let req: Request<Get, Domain> = Domains::get(domain);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}

#[test]
fn delete_produces_correct_request() {
    before();

    let domain = "example.com";
    let correct_url = format!("https://api.digitalocean.com/v2/domains/{}", domain);

    let req: Request<Delete, ()> = Domains::delete(domain);
    info!("{:#?}", req);

    assert_eq!(req.url.as_str(), correct_url);
    assert_eq!(req.body, Value::Null);
}
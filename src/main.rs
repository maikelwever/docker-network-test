extern crate trust_dns_resolver;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

use core::time::Duration;
use std::io::{Result, Error, ErrorKind};


fn do_lookup() -> Result<()> {
    let mut opts = ResolverOpts::default();

    // The actual runtime timeout is not even close to 2 seconds,
    // it takes about 12 seconds in a container with broken iptables rules.
    opts.timeout = Duration::from_secs(2);
    opts.attempts = 1;
    let resolver = Resolver::new(ResolverConfig::cloudflare(), opts)?;
    let response = resolver.ipv4_lookup("cloudflare.com.")?;

    match response.iter().next() {
        Some(_) => Ok(()),
        None => Err(Error::new(ErrorKind::Other, "No results returned")),
    }
}

fn main() {
    match do_lookup() {
        Ok(_) => { println!("1"); },
        Err(_) => { println!("0"); }
    };
}

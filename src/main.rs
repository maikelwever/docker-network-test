extern crate trust_dns_resolver;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;


fn do_lookup() -> Result<(), std::io::Error> {
    let resolver = Resolver::new(ResolverConfig::cloudflare(), ResolverOpts::default())?;
    let response = resolver.lookup_ip("cloudflare.com.")?;

    let _ = response.iter().next().expect("no addresses returned");

    Ok(())
}

fn main() {
    match do_lookup() {
        Ok(_) => { println!("1"); },
        Err(_) => { println!("0"); }
    };
}

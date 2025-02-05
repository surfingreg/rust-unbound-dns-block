//!
//! example
//!
//!

/// main
fn main() {

    //let allowed_top_domains = vec!["co", "azureedge", ];
    let url = r"https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts";

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(domains) = rust_unbound_dns_block::top_domain_list(url).await {
                //for d in &domains {
                //    println!("{d}");
                //}
                //println!("unique top domains: {}", rust_unbound_dns_block::top_level_domain_count(&domains));
                rust_unbound_dns_block::print_unbound(&domains);
            }

        })

}
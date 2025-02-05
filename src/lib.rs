//! lib.rs
//!
//! Return a naive list of 2-part top level domains extracted from:
//! https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts
//! or similar.
//!

use itertools::Itertools;

///
/// vector of two-part top-level domains
///
/// Converts:
///     0.0.0.0 appnexus.net
/// to:
///     local-zone: "appnexus.net" redirect
///     local-data: "appnexus.net A 0.0.0.0"
///
///
pub async fn top_domain_list(url:&str)-> Option<Vec<String>> {

    // web request
    if let Ok(body) = reqwest::get(url).await {

        // extract the web response as text
        if let Ok(text) = body.text().await {

            let lines = text.split("\n");

            let mut domains:Vec<String> = lines
                .filter(|l| !l.trim().starts_with("#"))
                .filter(|l| l.trim().starts_with("0.0.0.0"))
                .map(|l| {
                    // remove comments on the same line
                    match l.split_once("#") {
                        Some((not_comment, _))=>not_comment.trim(),
                        None => l
                    }
                })
                // take the last contiguous string to be domain after whitespace after 0.0.0.0
                .filter_map(|l| l.split_ascii_whitespace().last())

                /*

                TODO: extract top level domains; but since my wife insists on using
                 twitter, this won't fly for now--among other reasons.

                */

                .filter_map(|subdomain|{
                    // extract the top-level domain
                    // for now only use 2-part top-level domains. Ignore the others. TLDExtract is super slow.
                    let mut parts:Vec<&str> = subdomain.split(".").collect();
                    if parts.len() >= 2 {
                        parts.reverse();
                        // todo: use TLD if speed isn't a factor
                        //parts.truncate(2);
                        //assert_eq!(2, parts.len());
                        parts.reverse();
                        let parts = parts.join(".");
                        Some(parts)
                    } else {
                        None
                    }

                    //let tld_extractor = tldextract::TldExtractor::new(TldOption::default().cache_path(".tld_cache"));
                    //if let Ok(tld_result) = tld_extractor.extract(subdomain) {
                    //    //println!("{:?}", tld_result);
                    //    if let Some(subdomain) = tld_result.subdomain {
                    //        Some(subdomain)
                    //    }else {
                    //        None
                    //    }
                    //} else {
                    //    //false
                    //    None
                    //}

                })
                .filter(|f|
                    // account for the line "0.0.0.0 0.0.0.0" in the hosts file
                    if *f != "0.0" {
                        true
                    } else {false}
                )
                .collect();

            domains.sort();
            let unique_domains = domains.into_iter().unique().collect();

            // TODO: remove duplicates, particularly due to having removed subdomains


            Some(unique_domains)
        } else { None }
    }
    else { None }
}

pub fn top_level_domain_count(domains:&Vec<String>)->usize {
    domains.len()
}

///
/// print the domain list in Unbound a-records.conf format
///
/// local-zone: "facebook.com" redirect
/// local-data: "facebook.com A 0.0.0.0"
///
pub fn print_unbound(domains:&Vec<String>){

    for d in domains.iter(){

        let line1 = format!("local-zone: \"{}\" redirect", d);
        let line2 = format!("local-data: \"{} A 0.0.0.0\"", d);

        println!("{}\n{}\n", line1, line2);

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        //let allowed_top_domains = vec!["co", "azureedge", ];
        let url = r"https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts";

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                if let Some(domains) = top_domain_list(url).await {
                    //for d in &domains {
                    //    println!("{d}");
                    //}
                    print_unbound(&domains);
                    println!("unique top domains: {}", top_level_domain_count(&domains));
                }

            })
    }


}
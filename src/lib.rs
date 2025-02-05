//! lib.rs
//!
//! Return a naive list of 2-part top level domains extracted from:
//! https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/fakenews/hosts
//! or similar.
//!

///
/// vector of two-part top-level domains
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
                .filter_map(|subdomain|{
                    // extract the top-level domain
                    // for now only use 2-part top-level domains. Ignore the others. TLDExtract is super slow.
                    let mut parts:Vec<&str> = subdomain.split(".").collect();
                    if parts.len() == 2 {
                        parts.reverse();
                        // todo: use TLD if speed isn't a factor
                        parts.truncate(2);
                        parts.reverse();
                        parts.join(".");
                        Some(subdomain.to_string())
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
                .collect();

            domains.sort();

            Some(domains)
        } else { None }
    }
    else { None }
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
                }

            })
    }


}
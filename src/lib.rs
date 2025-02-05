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

    if let Ok(body) = reqwest::get(url).await {
        if let Ok(text) = body.text().await {
            //println!("body = {text:?}");
            let lines = text.split("\n");

            //let lines:Vec<&str> =
            let mut domains:Vec<String> = lines

                .filter(|l| !l.trim().starts_with("#"))
                .filter(|l| l.trim().starts_with("0.0.0.0"))
                .map(|l| {
                    // remove comments on the same line
                    if let Some((not_comment, _)) = l.split_once("#") {
                        not_comment.trim()
                    } else {
                        l
                    }
                })
                .filter_map(|l| {
                    let l = l.split_ascii_whitespace();
                    if let Some(l) = l.last() {
                        Some(l.trim())
                    }else{
                        None
                    }
                })
                .filter_map(|subdomain|{

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

            for d in &domains {
                println!("{d}");
            }
            Some(domains)
        } else {
            None
        }
    }
    else {
        None
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
                top_domain_list(url).await;
            })
    }
}
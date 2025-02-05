# rust-unbound-block
Convert @StevenBlack's hosts list to unbound a-records.conf format
to serve as a sinkhole for ad and other unfriendly domains. This 
filters his list down to top-level domains and blocks all. It's also 
currently naive in that it only looks at 2-part top domains. So
spamsite.co.uk will fall through the cracks. Same with badsite.azure.com.
It would be easy and slow to enable the tldextract crate to do this properly.

Unique subdomains as of 2/5/2025:  
123569  
(not sure why there's one more than unified+fakenews: http://sbc.io/hosts/alternates/fakenews/hosts)

## usage
> ./cargo_audit_fix.sh; cargo audit; cargo test; 

Into your unbound a-records.conf file:
> cargo run --example naive >> unbound/a-records.conf   

# rust-unbound-block
Convert @StevenBlack's hosts list to the unbound a-records.conf format.

Unique subdomains as of 2/5/2025:  
123568  
(unified+fakenews: http://sbc.io/hosts/alternates/fakenews/hosts)

## usage
> ./cargo_audit_fix.sh; cargo audit; cargo test; 

Into your unbound a-records.conf file:
> cargo run --example naive >> unbound/a-records.conf   

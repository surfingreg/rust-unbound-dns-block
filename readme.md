# rust-unbound-dns-block
Convert @StevenBlack's hosts list to the unbound a-records.conf format.

Unique subdomains as of 2/5/2025:  
123568  
(unified+fakenews: http://sbc.io/hosts/alternates/fakenews/hosts)

## usage
> ./cargo_audit_fix.sh; cargo audit; cargo test; 

Into your unbound a-records.conf file:
> cargo run --example naive >> unbound/a-records.conf   

## Background

I use https://github.com/MatthewVance/unbound-docker to run Unbound
in Docker. Then I run this script as a cron job to refresh my local 
block list nightly from https://github.com/StevenBlack/hosts/tree/master?tab=readme-ov-file.

update.sh
```
# clear old temp
rm a-records.conf
# backup active
cp unbound/a-records.conf a-records.conf_$(date "+%Y%m%d")
# use small original as baseline
cp a-records.conf.original a-records.conf
# run this program
cd rust-unbound-dns-block
git fetch --all
git pull origin main
cargo run --example naive >> a-records.conf
cd ..
rm unbound/a-records.conf
# move to active and restart docker
mv a-records.conf unbound/a-records.conf
ls -al unbound/a-records.conf #yep refreshed
./start.sh
```

start.sh
```
docker run \
  --name=unbound \
  --volume=[my path]/etc_unbound:/opt/unbound/etc/unbound/ \
  --publish=53:53/tcp \
  --publish=53:53/udp \
  --restart=unless-stopped \
  --detach=true \
  mvance/unbound:1.22.0
```

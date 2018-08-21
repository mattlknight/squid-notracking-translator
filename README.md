# squid-notracking-translator
- Rust application to translate dnsmasq based https://github.com/notracking/hosts-blocklists to squid domain acl blocklist
- Example Use Case:
  - Clone the notracking repo, `sudo git clone https://github.com/notracking/hosts-blocklists /opt/hosts-blocklists`
  - Compile the squid-translator binary per below instructions
  - Symlink the stripped binary to /usr/bin/ `sudo ln -s /opt/squid-notracking-translator/target/release/squid-translator /usr/bin/`
  - Update squid.conf with acl config, edit crontab to update notracking list, translate to squid acl, and then reload squid config
```text
localadmin@squid:~$ cat /etc/squid/squid.conf
[...]
acl notracking dstdomain "/etc/squid/notracking_blocklist.acl"
http_access deny notracking
http_access deny notracking CONNECT
[...]
localadmin@squid:~$ sudo crontab -l
50 * * * * cd /opt/hosts-blocklists/ && git pull > /opt/git_pull.log 2>&1 
55 * * * * squid-translator -d /opt/hosts-blocklists/domains.txt -s /etc/squid/notracking_blocklist.acl
0 * * * * systemctl reload squid.service
```
- About squid-translator
```text
loc target/release/squid-translator
--------------------------------------------------------------------------------
 Language             Files        Lines        Blank      Comment         Code
--------------------------------------------------------------------------------
 Rust                     1          105           12            9           84
--------------------------------------------------------------------------------
 Total                    1          105           12            9           84
--------------------------------------------------------------------------------

file target/release/squid-translator
target/release/squid-translator: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=82026bcabea94f4752c5238e8fd210f632a14e28, stripped

size target/release/squid-translator
   text    data     bss     dec     hex filename
   1875027  141224    4456 2020707  1ed563 target/release/squid-translator

ls -alh target/release/squid-translator
   [...] 2.0M Aug 21 10:59 target/release/squid-translator
```
- Uses CLI arguments and provides help
```text
localadmin@squid:~$ sudo squid-translator -h
Squid NoTracking Translator 1.0
Matthew Knight <mattlknight@gmail.com>
Converts https://github.com/notracking/hosts-blocklists to Squid compatible blocklist

USAGE:
    squid-translator --domain_blocklist <DOMAIN_BLOCKLIST> --squid_blocklist <SQUID_BLOCKLIST>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --domain_blocklist <DOMAIN_BLOCKLIST>    Sets the filename for the dnsmasq compatible domain blocklist
    -s, --squid_blocklist <SQUID_BLOCKLIST>      Sets the filename for the translated Squid compatible domain blocklist
```

## Building/Compiling

### Requires:
- Rust Nightly from https://rustup.rs/
- `sudo apt install build-essential` for Ubuntu based platform, or equivalent

### Build
- Clone this repository
  - `git clone https://github.com/mattlknight/squid-notracking-translator.git && cd squid-notracking-translator`
- Compile for release
  - `cargo build --release`
- Strip the build target
  - `strip target/release/squid-translator`
- Copy the compiled target to wherever
  - `cp target/release/squid-translator <wherever>`


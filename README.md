# squid-notracking-translator
- Rust application to translate dnsmasq based https://github.com/notracking/hosts-blocklists to squid domain acl blocklist
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


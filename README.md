# squid-notracking-translator
- Rust application to translate dnsmasq based https://github.com/notracking/hosts-blocklists to squid domain acl blocklist

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


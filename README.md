# dohc-rs

[![Actions Status](https://github.com/thekuwayama/dohc-rs/workflows/CI/badge.svg)](https://github.com/thekuwayama/dohc-rs/actions?workflow=CI)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/dohc-rs/master/LICENSE.txt)

`dohc-rs` is DNS over HTTPS Client implemented by Rust.


## Usage

You can build and run `dohc-rs` with the following:

```bash
$ git clone git@github.com:thekuwayama/dohc-rs.git

$ cd dohc-rs

$ cargo build

$ ./target/debug/dohc-rs --help
dohc 0.1.0
DNS over HTTPS Client implemented by Rust

USAGE:
    dohc-rs [OPTIONS] <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --type <type>    Query Type (either a numeric value or text)

ARGS:
    <name>    Query Name
```

```bash
$ ./target/debug/dohc-rs cloudflare.com | jq '.'
{
  "Status": 0,
  "TC": false,
  "RD": true,
  "RA": true,
  "AD": true,
  "CD": false,
  "Question": [
    {
      "name": "cloudflare.com.",
      "type": 1
    }
  ],
  "Answer": [
    {
      "name": "cloudflare.com.",
      "type": 1,
      "TTL": 153,
      "data": "104.17.175.85"
    },
    {
      "name": "cloudflare.com.",
      "type": 1,
      "TTL": 153,
      "data": "104.17.176.85"
    }
  ]
}
```

```bash
$ ./target/debug/dohc-rs one.one.one.one --type cname | jq '.'
{
  "Status": 0,
  "TC": false,
  "RD": true,
  "RA": true,
  "AD": false,
  "CD": false,
  "Question": [
    {
      "name": "one.one.one.one.",
      "type": 5
    }
  ],
  "Authority": [
    {
      "name": "one.one.one.",
      "type": 6,
      "TTL": 3600,
      "data": "fred.ns.cloudflare.com. dns.cloudflare.com. 2033516295 10000 2400 604800 3600"
    }
  ]
}
```


## License

The CLI is available as open source under the terms of the MIT License.

extern crate clap;

use clap::{App, Arg};

mod doh;

fn main() {
    let cli = App::new("dohc")
        .version("0.1.0")
        .about("DNS over HTTPS Client implemented by Rust")
        .arg(Arg::with_name("name").help("Query Name").required(true))
        .arg(
            Arg::with_name("type")
                .help("Query Type (either a numeric value or text)")
                .short("t")
                .long("type")
                .takes_value(true),
        );
    let matches = cli.get_matches();
    let name = matches
        .value_of("name")
        .expect("Falid: not specify domain name");
    let qt = matches.value_of("type").unwrap_or("A");

    let result = doh::resolve(name, qt).expect("Faild: resolve domain name");
    println!("{}", result);
}

#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use dohc::doh;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
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

    let result = doh::resolve(name, qt).await?;
    println!("{}", result);

    Ok(())
}

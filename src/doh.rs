extern crate failure;
extern crate reqwest;

use reqwest::header::ACCEPT;

pub fn resolve(name: &str, query_type: &str) -> Result<String, failure::Error> {
    let client = reqwest::Client::builder().use_rustls_tls().build()?;

    let s = client
        .get(&gen_uri(name, query_type))
        .header(ACCEPT, "application/dns-json")
        .send()
        .and_then(|mut response| response.text())?;

    Ok(s)
}

fn gen_uri(name: &str, query_type: &str) -> String {
    format!(
        "https://cloudflare-dns.com/dns-query?name={}&type={}",
        name, query_type
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_uri() {
        assert_eq!(
            gen_uri("example.com", "A"),
            "https://cloudflare-dns.com/dns-query?name=example.com&type=A"
        );
    }
}

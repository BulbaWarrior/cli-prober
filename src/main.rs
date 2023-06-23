use std::time::Duration;

use clap::Parser;
use reqwest::Client;

#[derive(clap::Parser)]
struct Args {
    delay: u64,
    url: String, //better validate the url here, but since it is required to keep probing on invalid url, we take any string here
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    loop {
        let mut output = format!("checking {}. ", &args.url);
        let res = probe(&args.url).await;
        output.push_str(&format!("Result: {:?}", res));
        println!("{output}");

        tokio::time::sleep(Duration::from_secs(args.delay)).await
    }
}

#[derive(Debug)]
enum ProbeResult {
    Ok(u16),
    Err(u16),
    UrlError,
}

async fn probe(url: &str) -> ProbeResult {
    let client = Client::new();

    let request = match client.get(url).build() {
        Ok(x) => x,
        Err(_) => return ProbeResult::UrlError,
    };

    let resp = client
        .execute(request)
        .await
        .expect("error sending request");

    match resp.error_for_status() {
        Ok(res) => {
            let status = res.status().as_u16();
            ProbeResult::Ok(status)
        }
        Err(status_error) => ProbeResult::Err(
            status_error
                .status()
                .expect("guranteed to be status")
                .as_u16(),
        ),
    }
}

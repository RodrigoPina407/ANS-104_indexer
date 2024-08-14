mod data_item;
mod http_client;

use clap::{command, Parser};
use data_item::DataItem;
use http_client::HttpClient;
use serde_json::{Result, Value};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// ANS-104 tx id
    tx: String,
}

// test tx id NY323dfv59V0hEFifbBiNOkfjAGP_OCpKZbMgpjFcEg; tafV876t7ZFKeY0ezDc-Q_-jUUQc72HB9g3Pjm5HPn4
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let Cli { tx } = cli;

    let b64_encoder = base64::engine::general_purpose::URL_SAFE_NO_PAD;

    let base_url = "https://arweave.net/tx/";

    let url = format!("{base_url}{tx}");

    let client = HttpClient::new();

    if let Ok((status_code, response)) = client.get(&url).await {
        if status_code == 200 {
            if let Ok(data_item) = parse_data_items(response) {
                if data_item.verify_signature(&b64_encoder) {
                    
                    let indexable = serde_json::to_string(&data_item.get_decoded_data(&b64_encoder)).unwrap();

                    println!("{indexable:#}");

                }
            }
        }
    };
}

fn parse_data_items(response: Value) -> Result<DataItem> {
    serde_json::from_value::<DataItem>(response)
}

mod data_item;
mod http_client;

use base64::{engine::GeneralPurpose, Engine};
use clap::{command, Parser};
use data_item::{DataItem, DataItemIndexed};
use http_client::HttpClient;
use serde_json::{Result, Value};
use sha2::{Digest, Sha256};

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
                if verify_signature(&data_item, &b64_encoder) {
                    let mut decoded_tags = Vec::new();
                    for tag in data_item.tags() {
                        if let Ok(decoded_tag) = tag.get_b64_decoded(&b64_encoder) {
                            decoded_tags.push(decoded_tag);
                        } else {
                            println!("Failed to decode tag...");
                        }
                    }

                }
            }
        }
    };
}

fn parse_data_items(response: Value) -> Result<DataItem> {
    serde_json::from_value::<DataItem>(response)
}

fn verify_signature(data_item: &DataItem, b64_encoder: &GeneralPurpose) -> bool {
    if let Ok(decoded_sig) = b64_encoder.decode(data_item.signature()) {
        let mut hasher: Sha256 = Sha256::new();

        hasher.update(decoded_sig);

        let encoded_id = b64_encoder.encode(hasher.finalize());

        return &encoded_id == data_item.id();
    }

    false
}

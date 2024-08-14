mod data_item;
mod http_client;

use clap::{command, Parser};
use data_item::DataItem;
use http_client::HttpClient;
use rusqlite::{Connection, Row};
use serde_json::Value;

static BASE_URL: &str = "https://arweave.net/tx/";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// ANS-104 tx id
    tx: String,

    /// Fetch tx data
    #[arg(short = 'd', long)]
    get_data: bool,
}

// test tx id NY323dfv59V0hEFifbBiNOkfjAGP_OCpKZbMgpjFcEg; tafV876t7ZFKeY0ezDc-Q_-jUUQc72HB9g3Pjm5HPn4

fn init_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("items.db")?;

    conn.execute(
        "create table if not exists tx_data (
             id text primary key,
             tags JSON not null,
             data BLOB
         )",
        rusqlite::params![],
    )?;

    Ok(conn)
}
#[tokio::main]
async fn main() {
    let db_conn = match init_db() {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to connect to DB");
            return;
        }
    };

    let cli = Cli::parse();

    let Cli { tx, get_data } = cli;

    let b64_encoder = base64::engine::general_purpose::URL_SAFE_NO_PAD;

    let url = format!("{BASE_URL}{tx}");

    let client = HttpClient::new();

    if let Ok((status_code, response)) = client.get(&url).await {
        if status_code == 200 {
            let data: Value = match response.json().await {
                Ok(d) => d,
                Err(e) => {
                    println!("{e}");
                    return;
                }
            };

            if let Ok(data_item) = parse_data_items(data) {
                if data_item.verify_signature(&b64_encoder) {
                    let indexable = data_item.get_decoded_metadata(&b64_encoder, &client).await;

                    if db_conn
                        .execute(
                            "INSERT INTO tx_data (id, tags, data) VALUES (?1, ?2, ?3)",
                            (&indexable.tx(), &indexable.tags(), &indexable.data()),
                        )
                        .is_ok()
                    {
                        println!("Tx: {} indexed", indexable.tx());
                    }

                    let _ = select_tx(&db_conn, &tx, get_data);
                }
            }
        }
    };
}

fn select_tx(db_conn: &Connection, id: &str, fetch_data: bool) -> Result<(), ()> {
    let mut query = "SELECT id, tags FROM tx_data where id = ?1";

    if fetch_data {
        query = "SELECT * FROM tx_data where id = ?1"
    }

    if let Ok(mut statement) = db_conn.prepare(query) {

        let closure = |row: &Row| {
            let tx: String = row.get(0)?;
            let tags: Value = row.get(1)?;
            let mut data = String::default();

            if fetch_data {
                data = row.get(2)?;
            }

            println!("tx: {tx}");
            println!("tags: {tags:#}");
            println!("data: {data:#}");

            Ok(())
        };

        if statement.query_row([id], closure).is_ok() {
            return Ok(());
        };

    }

    Err(())
}

fn parse_data_items(response: Value) -> serde_json::Result<DataItem> {
    serde_json::from_value::<DataItem>(response)
}

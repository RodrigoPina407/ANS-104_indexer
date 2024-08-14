use base64::{engine::GeneralPurpose, Engine};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[warn(dead_code)]
#[derive(Debug, Deserialize, Getters)]
pub struct Tag {
    name: String,
    value: String,
}

impl Tag {
    pub fn get_b64_decoded(&self, b64_encoder: &GeneralPurpose) -> Result<TagDecoded, ()> {
        if let (Ok(name_u8), Ok(value_u8)) = (
            b64_encoder.decode(self.name()),
            b64_encoder.decode(self.value()),
        ) {
            if let (Ok(name), Ok(value)) = (String::from_utf8(name_u8), String::from_utf8(value_u8))
            {
                return Ok(TagDecoded { name, value });
            }
        }

        Err(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Getters)]
pub struct DataItem {
    format: usize,
    last_tx: String,
    owner: String,
    target: String,
    tags: Vec<Tag>,
    data: String,
    signature: String,
    id: String,
    quantity: String,
    data_size: String,
    data_root: String,
    data_tree: Value,
    reward: String,
}

impl DataItem {
    pub fn verify_signature(&self, b64_encoder: &GeneralPurpose) -> bool {
        if let Ok(decoded_sig) = b64_encoder.decode(self.signature()) {
            let mut hasher: Sha256 = Sha256::new();

            hasher.update(decoded_sig);

            let encoded_id = b64_encoder.encode(hasher.finalize());

            return &encoded_id == self.id();
        }

        false
    }

    pub fn get_decoded_data(&self, b64_encoder: &GeneralPurpose) -> DataItemDecoded {
        let mut decoded_tags = Vec::new();
        for tag in self.tags() {
            if let Ok(decoded_tag) = tag.get_b64_decoded(b64_encoder) {
                decoded_tags.push(decoded_tag);
            } else {
                println!("Failed to decode tag...");
            }
        }

        DataItemDecoded {
            tx: self.id().to_owned(),
            tags: decoded_tags,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DataItemDecoded {
    tx: String,
    tags: Vec<TagDecoded>,
}

impl DataItemDecoded {}

#[derive(Debug, Serialize)]
pub struct TagDecoded {
    name: String,
    value: String,
}

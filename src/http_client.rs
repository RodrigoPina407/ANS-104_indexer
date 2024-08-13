use serde_json::Value;

#[derive(Debug)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: reqwest::Client::new(),
        }
    }

    /// Send HTTP Get request and return the response as json
    pub async fn get(
        &self,
        url: &str
    ) -> Result<(reqwest::StatusCode, Value), reqwest::Error> {
        let res = self.client.get(url).send().await?;
        let status: reqwest::StatusCode = res.status();
        let body = res.json().await?;
        Ok((status, body))
    }

  /*   pub async fn post(
        &self,
        url: &str,
        body: &str,
        headers: HeaderMap,
    ) -> Result<(reqwest::StatusCode, Value), Error> {
        let res = self
            .client
            .post(url)
            .body(body.to_owned())
            .headers(headers)
            .send()
            .await?;
        let status: reqwest::StatusCode = res.status();
        println!("Status: {:?}", status);
        let body = res.json().await?;
        Ok((status, body))
    } */
}



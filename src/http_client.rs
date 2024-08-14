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
        url: &str,
    ) -> Result<(reqwest::StatusCode, reqwest::Response), reqwest::Error> {
        let res: reqwest::Response = self.client.get(url).send().await?;
        let status: reqwest::StatusCode = res.status();

        Ok((status, res))
    }
}

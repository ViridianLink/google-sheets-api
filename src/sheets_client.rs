use crate::{Error, Result};
use reqwest::{Client, ClientBuilder, RequestBuilder, Url};
use serde::de::DeserializeOwned;

pub struct SheetsClientBuilder {
    api_key: String,
}

impl SheetsClientBuilder {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    pub fn build(self) -> Result<SheetsClient> {
        SheetsClient::new(self.api_key)
    }
}

pub struct SheetsClient {
    api_key: String,
    client: Client,
}

impl SheetsClient {
    pub fn new(api_key: String) -> Result<Self> {
        let client = ClientBuilder::new().build()?;

        Ok(SheetsClient { api_key, client })
    }

    pub async fn get<T>(&self, url: impl Into<Url>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut url = url.into();
        url.query_pairs_mut().append_pair("key", &self.api_key);

        let reqwest = self.client.get(url);
        self.process_response::<T>(reqwest).await
    }

    pub async fn process_response<T>(&self, reqwest: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = reqwest.send().await?;

        if let Some(hv) = response.headers().get("Content-Type") {
            if !hv
                .to_str()
                .map(|s| s.starts_with("application/json"))
                .unwrap_or(false)
            {
                return Err(Error::InvalidContentType(hv.to_owned()));
            }
        }

        response.json::<T>().await.map_err(Error::from)
    }
}

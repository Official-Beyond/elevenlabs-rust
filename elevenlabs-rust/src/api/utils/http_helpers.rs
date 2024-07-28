use reqwest::{Client, RequestBuilder};

pub fn create_request(client: &Client, method: reqwest::Method, url: &str) -> RequestBuilder {
    client.request(method, url)
          .header("Accept", "application/json")
          .header("Content-Type", "application/json")
}

use reqwest::Client;

use crate::api_client::{methods::HttpMethod, request::ApiRequest};

pub async fn send(request: ApiRequest) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let builder = match request.method {
        HttpMethod::Get => client.get(&request.url),    
        HttpMethod::Post => client.post(&request.url),
        HttpMethod::Put => client.put(&request.url),
        HttpMethod::Delete => client.delete(&request.url),
        HttpMethod::Patch => client.patch(&request.url),
    };

    let response = builder.send().await?.text().await?;

    Ok(response)
}

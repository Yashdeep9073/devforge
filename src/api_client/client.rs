use reqwest::Client;

use crate::api_client::{ApiRequest, HttpMethod};

pub async fn send(request: ApiRequest) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let builder = match request.method {
        HttpMethod::Get => client.get(&request.url),

        HttpMethod::Post => {
            let mut req = client.post(&request.url);

            if let Some(body) = request.body {
                req = req.header("Content-Type", "application/json").body(body);
            }

            req
        }

        HttpMethod::Put => client.put(&request.url),

        HttpMethod::Delete => client.delete(&request.url),

        HttpMethod::Patch => client.patch(&request.url),
    };

    let response = builder.send().await?.text().await?;

    Ok(response)
}

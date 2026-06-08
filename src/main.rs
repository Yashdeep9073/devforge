mod api_client;

use api_client::{ApiRequest, HttpMethod, send};

#[tokio::main]
async fn main() {
    // let request = ApiRequest {
    //     method: HttpMethod::Get,
    //     url: String::from("https://jsonplaceholder.typicode.com/users"),
    // };

    let request = ApiRequest {
        method: HttpMethod::Post,
        url: String::from("https://jsonplaceholder.typicode.com/posts"),
        body: Some(String::from(r#"{"id":1}"#)),
    };

    match send(request).await {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("{}", err),
    }
}

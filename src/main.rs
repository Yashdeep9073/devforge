mod api_client;

#[tokio::main]
async fn main() {
    match api_client::get("https://jsonplaceholder.typicode.com/users").await {
        Ok(response) => {
            println!("Response: {}", response);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

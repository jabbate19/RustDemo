// Let's make a web request to a website!
//
// We're going to need the reqwest and tokio crates
// So we can use async and have an easier web request process
use reqwest::Client;

// This allows the main function to be async, which isn't allowed by default
#[tokio::main]
async fn main() {
    let url = "http://example.com";
    let client = Client::new();
    let website_data = client.get(url).send().await.unwrap().text().await.unwrap();
    println!("{}", website_data);

    // Now this has a lot of unwrap() in there. What if send() fails? What if text() fails?
    // For this, we could use match cases
    let website_data = match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => text,
            Err(x) => format!("Failed to get text: {}", x),
        },
        Err(x) => format!("Failed to send request: {}", x),
    };
    println!("{}", website_data);

    // You could also use unwrap_or_else, which will return the Ok value or whatever you want with
    // the Error
    let website_data = client
        .get(url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_else(|error| format!("Failed to get text: {}", error));
    println!("{}", website_data);
}

use reqwest::{Error, Client};
mod tender;
pub static secret_key: &str = "YOUR_API_KEY_HERE";

async fn call_api_and_print_json(amount: i32, curr: String) -> Result<(), Error> {
    let client = Client::new();
    let response = client
        .post("https://api.shift4.com/charges")
        .basic_auth(secret_key, Some(""))
        .form(&[
            ("amount", amount.to_string().as_str()),
            ("currency", &curr),
            ("customerId", "cust_DIEMETEXAo1TAkvuoGrqet0p"),
            ("card", "card_RCvnxQus55EjHue0vsGv3Y6c"),
            ("description", "Rust API amount argurment"),
        ])
        .send()
        .await?;

    let json_response = response.text().await?;
    println!("{}", json_response);

    Ok(())
}

#[tokio::main]
async fn main() {
    let amount = 765;
    let currency = tender::Currency::USD;
    if let Err(err) = call_api_and_print_json(amount,currency.to_string()).await {
        eprintln!("Error: {}", err);
    }
}



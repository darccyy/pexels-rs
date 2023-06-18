use std::env;

fn main() {
    dotenv::dotenv().ok();

    let api_key =
        env::var("PEXELS_API_KEY").expect("API key for api.pexels.com not defined in .env file");
}

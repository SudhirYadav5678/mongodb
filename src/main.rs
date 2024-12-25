use std::env;
use mongodb::{Client,options::{ClientOptions,ResolverConfig}};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>> {
    println!("Hello, world!");
    let client_url = env::var("MONGODB_URL").expect("You must set mongodb_url");
    let options=ClientOptions::parse_with_resolver_config(&client_url,ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    println!("Database");
    for name in client.list_database_names(None,Nome).await?{
        println!("- {}",name);
    }
    Ok({})
}

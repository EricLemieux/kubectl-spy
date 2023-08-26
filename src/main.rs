use clap::Parser;
use k8s_openapi::api::core::v1::Secret;
use kube::Client;
use std::collections::HashMap;
use std::error::Error;
use std::str;

mod kubernetes;

#[derive(Parser, Debug)]
struct Cli {
    secrets: Vec<String>,

    #[arg(default_value = "default", short, long)]
    namespace: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut secrets: HashMap<String, HashMap<String, String>> = HashMap::new();
    for secret_name in args.secrets {
        let secret = kubernetes::get_secret(
            Client::try_default().await?,
            secret_name.clone(),
            String::from(&args.namespace),
        )
        .await;

        let secret_key_value_pairs = get_secret_key_value_pairs(&secret);

        secrets.insert(secret_name.clone(), secret_key_value_pairs);
    }

    Ok(println!("{}", serde_json::to_string(&secrets)?))
}

fn get_secret_key_value_pairs(secret: &Secret) -> HashMap<String, String> {
    secret
        .clone()
        .data
        .unwrap_or_default()
        .iter()
        .map(|a| {
            (
                String::from(a.0),
                String::from(str::from_utf8(&a.1 .0).unwrap_or_default()),
            )
        })
        .collect()
}

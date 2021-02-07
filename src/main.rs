use kube::Client;
use std::collections::HashMap;
use std::str;
use structopt::StructOpt;

mod kubernetes;

#[derive(StructOpt, Debug)]
struct Cli {
    secrets: Vec<String>,
    #[structopt(default_value = "default", short = "n", long = "namespace")]
    namespace: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let client = Client::try_default().await.unwrap();

    let mut secrets: HashMap<String, HashMap<String, String>> = HashMap::new();
    for name in args.secrets.iter() {
        let secret = kubernetes::get_secret(
            client.clone(),
            String::from(name),
            String::from(&args.namespace),
        )
        .await;

        let mut values: HashMap<String, String> = HashMap::new();
        for data in secret
            .clone()
            .data
            .unwrap_or(std::collections::BTreeMap::new())
            .iter()
        {
            values.insert(
                String::from(data.0),
                String::from(str::from_utf8(&data.1 .0).unwrap()),
            );
        }

        secrets.insert(secret.clone().metadata.name.unwrap(), values);
    }

    println!("{}", serde_json::to_string(&secrets).unwrap());
}

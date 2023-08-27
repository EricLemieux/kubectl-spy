use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};

pub async fn get_secret(client: &Client, secret_name: &String, namespace: &String) -> Secret {
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);

    secrets.get(secret_name.as_str()).await.unwrap_or_else(|_| {
        panic!("Unable to locate secret {secret_name} in namespace {namespace}")
    })
}

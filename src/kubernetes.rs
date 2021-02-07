use k8s_openapi::api::core::v1::Secret;
use kube::{Api, Client};
use std::process::exit;

pub async fn get_secret(client: Client, secret_name: String, namespace: String) -> Secret {
    let secrets: Api<Secret> = Api::namespaced(client, &namespace);

    return match secrets.get(secret_name.as_str()).await {
        Ok(secret) => secret,
        Err(_e) => {
            eprintln!(
                "Unable to locate secret {:?} in namespace {:?}",
                secret_name, namespace
            );
            exit(1);
        }
    };
}

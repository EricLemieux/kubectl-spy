use kube::{
    Api,
    Client
};
use k8s_openapi::api::core::v1::Secret;

pub async fn get_secret(client: Client, secret_name: String, namespace: String) -> Secret {
    let secrets :Api<Secret> = Api::namespaced(client, &namespace);

    return secrets.get(secret_name.as_str()).await.unwrap();
}

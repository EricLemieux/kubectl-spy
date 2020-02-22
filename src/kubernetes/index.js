const k8s = require('@kubernetes/client-node');

/**
 * Authenticate with the kubernetes cluster using your kubectl config.
 *
 * @returns {CoreV1Api}
 */
function authenticate() {
    const kubeConfig = new k8s.KubeConfig();
    kubeConfig.loadFromDefault();

    return kubeConfig.makeApiClient(k8s.CoreV1Api);
}

/**
 * Get the secret object from the kubernetes cluster
 *
 * @param api
 * @param name
 * @param namespace
 * @returns {Promise<{response: http.IncomingMessage; body: V1Secret}>}
 */
function getSecret(api, name, namespace) {
    if (namespace === undefined) {
        namespace = 'default';
    }

    return api.readNamespacedSecret(name, namespace);
}

module.exports = {
    authenticate,
    getSecret,
};

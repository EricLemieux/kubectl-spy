const program = require('commander');
const base64 = require('./base64');
const kubernetes = require('./kubernetes');

(async function main() {
    try {
        program
            .version('1.0.0')
            .option('-n, --namespace <string>', 'namespace for the secrets')
            .parse(process.argv);

        // Authenticate with the kubernetes cluster, so that we can perform queries.
        const k8sApi = kubernetes.authenticate();

        const secrets = {};
        for (const secretName of program.args) {
            // Fetch the secret from the desired namespace
            const k8sSecret = await kubernetes.getSecret(k8sApi, secretName, program.namespace);

            Object.entries(k8sSecret.body.data).forEach(([key, value]) => {
                if (!secrets.hasOwnProperty(secretName)) {
                    secrets[secretName] = {};
                }

                secrets[secretName][key] = base64.decode(value);
            });
        }

        console.log(JSON.stringify(secrets));
    } catch (e) {
        console.error(e);
    }
})();

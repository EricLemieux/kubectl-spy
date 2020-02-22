# Kubernetes

Sample inputs for testing kubectl-spy

## Usage

Create the extra namespace for the database secret
```shell script
kubectl create namespace database
```

Install the secrets
```shell script
kubectl apply -f kubernetes/
```

Get the values from the secrets
```shell script
kubectl spy my-secret-name
kubectl spy database-secret -n database
```

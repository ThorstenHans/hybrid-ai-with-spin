# Hybrid AI with SpinKube and Fermyon Cloud

The sample application in this repository illustrates how you could build a Spin App (hosted on Kubernetes using [SpinKube](https://spinkube.dev)) and leverage Serverless AI capabilities provided by Fermyon Cloud.

## Prerequisites

To run this sample, you need:

- Spin CLI installed
  - The `cloud` plugin
  - The `cloud-gpu` plugin
  - The `kube` plugin
- Rust and its `wasm32-wasi` target installed
- The Kubernetes CLI `kubectl` installed
- Access to a Kubernetes cluster with [SpinKube](https://spinkube.dev) deployed 
- A free [Fermyon Cloud Account](https://cloud.fermyon.com)


## Deploy Cloud-GPU proxy

First, deploy the Cloud-GPU proxy to your Fermyon Cloud Account:

```bash
spin cloud-gpu init
```

Update necessary information in the [Runtime Configuration File](./llm.toml)

## Start the Spin App locally

```bash
spin up --build --runtime-config-file ./llm.toml
```

You're now able to browse the app on `localhost:3000` and leverage Serverless AI powered by Fermyon Cloud.

## Kubernetes Deployment


First, build and push the Spin App as OCI artifact:

```bash
spin registry push ttl.sh/hybrid-ai:12h --build
```

Update the deployment manifests:

```bash
spin kube scaffold --from ttl.sh/hybrid-ai:12h --runtime-config-file ./llm.toml
```

Deploy the manifests to your Kubernetes cluster:

```bash
kubectl apply -f ./manifests.yaml
```

If you have an Ingress Controller in place, you can deploy the `Ingress` rule to route all incoming requests to the Spin App. Alternatively, you can use `kubectl port-forwarding`.

```bash
kubectl apply -f ./ingress.yaml
```

# mob-backend

## building:

`cargo run`

## releasing:

Requires docker login with Digital Ocean API Key
Requires Kubernetes Config file from Digital Ocean

`./release.sh`

`kubectl --kubeconfig=./kube/mob-backend-kubeconfig.yaml`

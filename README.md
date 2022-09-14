# mob-backend

## building:

`cargo run`

## releasing:

Requires docker login with Digital Ocean API Key
Requires Kubernetes Config file from Digital Ocean

`./release.sh`

`kubectl --kubeconfig=./kube/mob-backend-kubeconfig.yaml`

`kubectl --kubeconfig=./kube/auth.yaml create deployment mob --image=registry.digitalocean.com/mob/backend:latest --port=8000`

`kubectl --kubeconfig=./kube/mob-backend-kubeconfig.yaml expose rc nginx --port=80 --target-port=8000`

`kubectl --kubeconfig=./kube/auth.yaml get svc -n ingress-nginx`

`kubectl --kubeconfig=./kube/auth.yaml apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/nginx-0.27.0/deploy/static/mandatory.yaml`

`kubectl --kubeconfig=./kube/auth.yaml expose deployment mob --type=LoadBalancer --port=80 --target-port=8000`
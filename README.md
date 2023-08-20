Hello App
===============

## Requirements
* Rust (nightly)
* Docker
* docker-compose

## Usage
```
# Run postgres
make dev

# Install rust tools
make install

# Run db migrations
export DATABASE_URL=postgresql://admin:password123@localhost:6500/users?application_name=hello-app
make migrate-up

# Run the server
make start-server

# Tests
cargo test
```

## Local Build and Test
```
#Build
docker build . -t hello-app


# Run on local docker
docker container run --net=hello-app_backnet -e DATABASE_URL="postgresql://admin:password123@postgres:5432/users?application_name=hello-app" --expose 8080 -p 3000:8080 hello-app
```

# Routes
- `GET` `/health` -> health check
  **Response:**
  ```
  {"message":"Health","status":"success"}
  ```
- `GET` `/hello/<username>` -> Hello Name
  **Response:**
  ```
  A:
  { “message”: “Hello, <username>! Your birthday is in N day(s)”}

  B:
  { “message”: “Hello, <username>! Happy birthday!” }
  ```

- `PUT` `/hello/<username>` -> Add or change the `<username>` with birthday date
  **Request Body:**
  ```
  {"dateOfBirth":"1970-01-01"}
  ```
  **Response:**
  ```
  204 No Content
  ```

- `GET` `/metrics` -> Return Prometheus metrics
  **Response:**
  ```
  api_http_requests_duration_seconds_count{endpoint="/metrics",method="GET",status="200"} 1
  # HELP api_http_requests_total Total number of HTTP requests
  # TYPE api_http_requests_total counter
  api_http_requests_total{endpoint="/metrics",method="GET",status="200"} 1
  ```

# Infrastructure AWS K8S 
**Terraform will create a EKS cluster with EKS blueprints**
```
cd infra/terraform
terraform init
terraform plan
terraform apply
```
# NGINX Ingress Controller 
## NGINX Ingress Controller reduces complexity, increases uptime, and provides better insights into app health and performance at scale.
Simplify Operations
NGINX Ingress Controller reduces tool sprawl through technology consolidation:

A universal Kubernetes-native tool for implementing API gateways, load balancers, and Ingress controllers at the edge of a Kubernetes cluster
The same data and control planes across any hybrid, multi-cloud environment
Tight and seamless integration with NGINX Service Mesh for unified app connectivity to, from, and within the cluster

- Install Nginx Ingress Controller via Helm
```
helm upgrade --install ingress-nginx ingress-nginx \
--repo https://kubernetes.github.io/ingress-nginx \
--namespace ingress-nginx --create-namespace
```

# Deployment
## The deployment could be implemented by using helm via CI/CD, or ArgoCD using kustomization(deployemnt/kustomize)
```
helm install hello-app deployment/helm/hello-app
```

## Avoiding downtime deployment
By default kubernetes uses the strategy used to replace old Pods by new ones with "RollingUpdate" is the default value.

But in case that K8S nodes using SPOT instances, the recomendation is to use Pod Disruption Budget.

The minAvailable parameter could be patch using kustomization base on environment.
```
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: hello-app-pdb
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: hello-app
```

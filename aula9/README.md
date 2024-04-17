# LimitRange
```sh
kubectl apply -f namespace.yaml

kubectl apply -f limitrange-pod.yaml
# Corrija o erro

kubectl apply -f limitrange-container.yaml
# Corrija o erro
```

# ResourceQuota

```sh
minikube start minikube start --driver docker --ports=8080:80,30082:30082 --extra-config=kubelet.housekeeping-interval=10s --kubernetes-version=v1.29 --extra-config=apiserver.enable-admission-plugins=ResourceQuota
```

# Monitoramento de Cluster com Prometheus e Grafana - Helm

```sh
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update

helm install latest prometheus-community/kube-prometheus-stack

kubectl port-forward svc/latest-grafana 8082:80
```

**Browser:** [http://localhost:8082](http://localhost:8082)
**User:** `admin`
**Password:** [prom-operator](https://github.com/prometheus-community/helm-charts/blob/main/charts/kube-prometheus-stack/values.yaml#L968)

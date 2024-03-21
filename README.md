# Getting Started
```sh
eval $(minikube -p minikube docker-env)
```


### Names

**RFC 1123 Label Names**
Some resource types require their names to follow the DNS label standard as defined in RFC 1123. This means the name must:
- contain at most 63 characters
- contain only lowercase alphanumeric characters or '-'
- start with an alphanumeric character
- end with an alphanumeric character

---

### Pods
Generate a pod yaml file
```sh
kubectl run --image nginx nginx --dry-run=client -oyaml nginx
kubectl run --image nginx nginx --dry-run=client -oyaml nginx > pod.yaml
```

Create a pod using the yaml file
```sh
kubectl apply -f pod.yaml
```

Get the pod
```sh
kubectl get pods -A
```

Get logs from the pod
```sh
kubectl logs pods/nginx
```

Get the pod's IP address
```sh
kubectl get pods -o wide
```

**DNS do Pod:**
```
<pod-ip-address>.<namespace>.pod.cluster.local
```

---

### Deployment

Create deployment file
```sh
kubectl create deployment --image nginx nginx --dry-run=client -oyaml > deployment.yaml
```

Update the deployment image
```sh
kubectl set image deployment/nginx-deployment nginx=nginx:1.16.1
```

Obter detalhes do deployment
```sh
kubectl describe deployment nginx-deployment
```

Rollback the deployment
```sh
kubectl rollout status deployment/nginx-deployment
```

```sh
kubectl rollout history deployment/nginx-deployment
kubectl rollout history deployment/nginx-deployment --revision=2

kubectl rollout undo deployment/nginx-deployment
kubectl rollout undo deployment/nginx-deployment --to-revision=2
```

Redimensionar o deployment
```sh
kubectl scale deployment/nginx-deployment --replicas=10
```

Autoescalar o deployment (Necessário habilitar o escalonamento automático horizontal de pods no cluster - HPA)
```sh
kubectl autoscale deployment/nginx-deployment --min=10 --max=15 --cpu-percent=80
```

---

### Services

**DNS do Service:**
```
<service-name>.<namespace>.svc.cluster.local
```

---

```sh
kubectl describe node <node-name>
```

Access etcd
```sh
kubectl -n kube-system exec -it etcd-minikube -- sh -c "ETCDCTL_API=3 ETCDCTL_CACERT=/var/lib/minikube/certs/etcd/ca.crt ETCDCTL_CERT=/var/lib/minikube/certs/etcd/server.crt ETCDCTL_KEY=/var/lib/minikube/certs/etcd/server.key etcdctl get /registry/deployments/default/nginx-deployment"
```

```sh
kubectl expose deployment/nginx-deployment --type=NodePort

minikube service --all
```

---

# Kubernetes API

Verbosidade lvl 6
```sh
kubectl get pods -v6
```

Obter o OpenAPI do cluster
```sh
kubectl proxy --port=8080
curl http://localhost:8080/openapi/v3 > openapi.json
```

Obter a lista de pods através da API Rest
```sh
curl http://127.0.0.1:8080/api/v1/namespaces/default/pods
```
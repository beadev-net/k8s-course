```sh
minikube start --driver docker  --ports=127.0.0.1:30080:30080

kubectl apply -f nodeport/

# Internal - Inside the pod
kubectl exec pod/nginx-clusterip -it sh

curl $(minikube ip):30080

###

# External - Open in browser
localhost:30080
```
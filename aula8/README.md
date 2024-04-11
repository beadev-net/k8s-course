# Service Account Example

```sh
kubectl apply -f serviceaccount-example.yaml
kubectl exec -it pod-nginx -n aula8 -- /bin/sh
```

Dentro do Pod, execute:
```sh
TOKEN=$(cat /var/run/secrets/kubernetes.io/serviceaccount/token)
APISERVER=https://kubernetes.default.svc
CACERT=/var/run/secrets/kubernetes.io/serviceaccount/ca.crt

curl --cacert $CACERT --header "Authorization: Bearer $TOKEN" -X GET $APISERVER/api/v1/namespaces/default/pods
```


# OIDC Example (OKTA)

### Requirements
- https://krew.sigs.k8s.io/docs/user-guide/setup/install/
- https://github.com/int128/kubelogin

**Dashboard:** https://dev-03131721-admin.okta.com/admin/dashboard

```sh
sh oidc.sh

kubectl apply -f oidc-example.yaml

kubectl get pods --user oidc-okta-user # 200

kubectl get cronjobs --user oidc-okta-user # 403
```

# X509 Client Certificate Example

```sh
openssl genrsa -out user2.key 2048

openssl req -new -key user2.key -out user2.csr -subj "/CN=user2/O=group1/O=group2"

cat user2.csr | base64 | tr -d '\n'

# Cole o valor no arquivo x509-example.yaml no campo request: 

kubectl apply -f x509-example.yaml

kubectl get csr

kubectl certificate approve user2
#kubectl certificate deny user2

kubectl get csr user2 -o jsonpath='{.status.certificate}' | base64 --decode > user2.crt

kubectl --client-key=user2.key --client-certificate=user2.crt get nodes
kubectl config set-credentials user2 --client-key user2.key --client-certificate user2.crt --embed-certs
kubectl config set-context user2 --cluster minikube --user user2

kubectl get nodes --user user2
```
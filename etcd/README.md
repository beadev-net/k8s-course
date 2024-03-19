Get etcd keys

```sh
kubectl -n kube-system exec -it etcd-minikube -- sh -c "ETCDCTL_API=3 ETCDCTL_CACERT=/var/lib/minikube/certs/etcd/ca.crt ETCDCTL_CERT=/var/lib/minikube/certs/etcd/server.crt ETCDCTL_KEY=/var/lib/minikube/certs/etcd/server.key etcdctl get \"\" --prefix=true --keys-only "
```

Get values from a key

```sh
kubectl -n kube-system exec -it etcd-minikube -- sh -c "ETCDCTL_API=3 ETCDCTL_CACERT=/var/lib/minikube/certs/etcd/ca.crt ETCDCTL_CERT=/var/lib/minikube/certs/etcd/server.crt ETCDCTL_KEY=/var/lib/minikube/certs/etcd/server.key etcdctl get /registry/services/specs/default/my-service"
```
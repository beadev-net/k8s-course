# K8s
- Cluster ✅
- Node (VM, Bare Metal) ✅
- Kubectl ✅
- Namespace ✅

# Control Plane:
- API Server ✅
- Controller Manager ✅
- etcd ✅
- Kubelet ✅
- Kube-Proxy ✅
- Container Runtime ✅
- Scheduler ✅
- Cloud Control Manager ❌

# Workloads

## Pods:
- Lifecycle ✅
- Quality of Service (QoS) | `Best Effort`, `Burstable`, `Guaranteed` ✅
- ImagePullPolicy | `Always`, `IfNotPresent`, `Never` ✅
- Resource Requests and Limits ✅
- Horizontal Pod Autoscaler ✅

## Deployments
- ReplicaSet ✅
- Rolling Update ✅
- Liveness Probe ✅
- Readiness Probe ✅
- Startup Probe ✅
- Paused Rollout ✅
- Rollback ✅

## StatefulSets ✅
- Pod Identity ✅
- Stable Network Identity ✅
- Ordered Deployment ✅
- Ordered Scaling ✅
- Ordered Deletion ✅
- Storage ✅
- Headless Service ✅

## Jobs
- Run to completion ✅
- Parallel or Sequential ✅
- Restart Policy ✅
- TTL ✅

## CronJobs
- Schedule Jobs ✅
- Timezone
- Concurrency Policy
- History Limit ✅

## DaemonSets ✅
- Run a copy of a Pod on each Node
- Monitoring, Logging, Networking, Storage, etc
- Node-specific tasks
- `kube-proxy` is a DaemonSet

# Services:
- ClusterIP ✅
- NodePort ✅
- Round Robin na prática gerenciado pelo K8s ✅
- LoadBalancer ✅
- ExternalName ✅
- Headless ✅

# Volumes:
- EmptyDir ✅
- HostPath ✅
- PersistentVolume ✅
- PersistentVolumeClaim ✅
- StorageClass ✅

# Configs:
- ConfigMap ✅
- Secret ✅

# Network
- Pod to Pod (FQDN - FULL QUALIFIED DOMAIN NAME - ip-do-pod.namespace.svc.cluster.local) ✅
- Pod to Service (FQDN - service.namespace.svc.cluster.local) ✅
- Ingress ✅
- Endpoint ✅
- EndpointSlices ✅
- IpTables ✅

# Security
- Pod Security Policy ❌
- Network Policy ❌
- RBAC ✅
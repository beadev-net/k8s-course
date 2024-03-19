# Pré-requisitos
- Ter o kubectl instalado e configurado para comunicar-se com seu cluster Kubernetes.
- Ter o Helm instalado (opcional, mas recomendado para instalar o Ingress Controller).
- Ter um cluster Kubernetes rodando.

## Passo 1: Criar Aplicação

### Definir um Deployment

Crie um arquivo chamado deployment.yaml para definir o Deployment da sua aplicação. Substitua sua-aplicacao pelo nome da sua aplicação e imagem-da-sua-aplicacao pela imagem Docker que deseja usar.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-app-deployment
spec:
  replicas: 2
  selector:
    matchLabels:
      app: my-app-replica
  template:
    metadata:
      labels:
        app: my-app-replica
    spec:
      containers:
      - name: my-app-container
        image: httpmock:05fdddc
        ports:
        - containerPort: 80
```

Aplique o Deployment:

```sh
kubectl apply -f deployment.yaml
```

### Criar um Service
Para expor sua aplicação dentro do cluster, crie um arquivo chamado service.yaml.
```yaml
apiVersion: v1
kind: Service
metadata:
  name: service-sua-aplicacao
spec:
  selector:
    app: sua-aplicacao
  ports:
  - protocol: TCP
    port: 80
    targetPort: 80
```

Aplique o Service:

```sh
kubectl apply -f service.yaml
```

---

## Passo 2: Ativar o Horizontal Pod Autoscaler (HPA)

### Criar HPA

Para criar um HPA, você precisa de métricas de CPU ou memória. Aqui está um exemplo de como criar um HPA para sua aplicação baseado no uso de CPU.

```sh
kubectl autoscale deployment sua-aplicacao --cpu-percent=50 --min=2 --max=5
```

Este comando criará um HPA que aumentará o número de réplicas da sua aplicação se o uso médio de CPU ultrapassar 50%, mantendo o número de réplicas entre 2 e 5.

---

## Passo 3: Configurar o Ingress

### Instalar um Ingress Controller  
Se ainda não tiver, instale um Ingress Controller. Um exemplo é o nginx-ingress:

```sh
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update
helm install nginx-ingress ingress-nginx/ingress-nginx
```

Habilitar o túnel do Minikube para acessar o Ingress Controller:
```sh
minikube tunnel
```

Habilitar o dashboard do Minikube:
```sh
minikube dashboard
```

Logs do Ingress Controller:
```sh
kubectl -n ingress-nginx logs -l app.kubernetes.io/instance=ingress-nginx
```
---

### Definir um Ingress Resource

Crie um arquivo ingress.yaml para definir as regras de acesso à sua aplicação.

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: ingress-sua-aplicacao
spec:
  rules:
  - http:
      paths:
      - path: /sua-aplicacao
        pathType: Prefix
        backend:
          service:
            name: service-sua-aplicacao
            port:
              number: 80
```

Aplique o Ingress com:
```sh
kubectl apply -f ingress.yaml
```

---

## Passo 4: Adicionar Volume Persistente

### Criar Persistent Volume Claim (PVC)

Crie um arquivo pvc.yaml para solicitar armazenamento persistente.

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: pvc-my-app
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 1Gi
```

Aplique o PVC com:
```sh
kubectl apply -f pvc.yaml
```

### Modificar o Deployment para Usar o PVC

Adicione a configuração do volume ao seu deployment.yaml, no campo spec do contêiner:

```yaml
volumes:
- name: storage
  persistentVolumeClaim:
    claimName: pvc-sua-aplicacao
```

E monte o volume dentro do contêiner:

```yaml
volumeMounts:
- mountPath: "/caminho/dentro/do/container"
  name: storage
```

Após modificar, aplique as alterações com:

```sh
kubectl apply -f deployment.yaml
```
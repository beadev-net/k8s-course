# Inicia o Minikube
```sh
sh start-minikube.sh
```

# Builda as imagens através dos Dockerfile's
```sh
sh build-images.sh
```

# Inicia as aplicações no Kubernetes
```sh
sh start-applications.sh
```

# Para conectar no Banco de Dados através do DBEAVER

##### MySQL MASTER
```sh
Host: localhost
Port: 31000
User: root
Password: your_password
```

##### MySQL REPLICA
```sh
Host: localhost
Port: 32000
User: root
Password: your_password
```

# Minikube Dashboard
```sh
minikube dashboard
```
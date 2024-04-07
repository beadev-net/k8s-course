#!bin/sh
minikube start --driver docker --ports=8080:80,30082:30082 --extra-config=kubelet.housekeeping-interval=10s --kubernetes-version=v1.29

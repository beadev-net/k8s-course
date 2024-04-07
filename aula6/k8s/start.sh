#!bin/sh
minikube start --driver docker --ports=8080:80,30082:30082,31150:31150 --extra-config=kubelet.housekeeping-interval=10s --kubernetes-version=v1.29


sleep 15

kubectl apply -f /mysql-master
sleep 5
kubectl apply -f /mysql-replica
sleep 5
kubectl apply -f /redis
sleep 5
kubectl apply -f /job-migration
sleep 5
kubectl apply -f /api
sleep 5
kubectl apply -f /frontend
sleep 5
kubectl apply -f /consumer
sleep 5
kubectl apply -f /job-csv



#!bin/sh

kubectl apply -f ./namespace.yaml
kubectl apply -f ./mysql-master
sleep 5
kubectl apply -f ./mysql-replica
sleep 5
kubectl apply -f ./redis
sleep 5
kubectl apply -f ./job-migration
sleep 5
kubectl apply -f ./api
sleep 5
kubectl apply -f ./frontend
sleep 5
kubectl apply -f ./consumer
sleep 5
kubectl apply -f ./job-csv



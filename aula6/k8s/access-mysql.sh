kubectl port-forward svc/mysql-replica-service -n aula6 32000:3306

kubectl port-forward svc/mysql-master-service -n aula6 31000:3306
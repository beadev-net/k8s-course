#!bin/sh

eval $(minikube -p minikube docker-env)

echo "Building images..."

echo "Building Job-Migration"
docker build ../job-migration -f ../job-migration/Dockerfile -t job-migration:1.0.0
echo "Finished Job-Migration"

echo "Building API"
docker build ../api -f ../api/Dockerfile -t api:1.0.0
echo "Finished API"

echo "Building Consumer"
docker build ../consumer -f ../api/Dockerfile -t consumer:1.0.0
echo "Finished Consumer"

echo "Building Job-CSV"
docker build ../job-csv -f ../job-csv/Dockerfile -t job-csv:1.0.0
echo "Finished Job-CSV"

echo "Building Frontend"
docker build ../frontend -f ../frontend/Dockerfile -t frontend:1.0.0 --build-arg API_URL=http://localhost:31150
echo "Finished Frontend"

echo "Finished!"
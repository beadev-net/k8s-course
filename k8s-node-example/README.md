- Clone this project
    - be in root of my project, Run `kubectl apply -f k8s`
    - Open new terminal Run `minikube dashboard` to open Kubernetes dashboard
    - Run `minikube service node-example` to check the node-app pod(container) is working or not
    - Check HPA demo live:
        - Open three different terminal
        - Install `watch` if not installed using `brew install watch`.
        - Terminal 1 -> Run `watch -n 1 kubectl get pods`
        - Terminal 2 -> Run `watch -n 1 kubectl get hpa`
        - Terminal 3 -> Run `ab -c 5 -n 1000 -t 100000 http://192.168.99.100:30001/`. Used Apache Benchmark here.
        - You can see authoscalling of pods in Terminal-1.






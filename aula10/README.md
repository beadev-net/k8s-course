```sh
helm create my-first-chart
cd my-first-chart
helm install my-first-chart . --dry-run
helm ls -A
```

```sh
helm template .
helm upgrade --install my-first-chart .
```

```sh 
helm uninstall my-first-chart
```
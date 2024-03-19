# Updating a deployment image
```sh
kubectl set image deployment/nginx-deployment nginx=nginx:1.16.1
```

to see rollout
```sh
kubectl rollout status deployment/nginx-deployment
```


# Label selector updates
It is generally discouraged to make label selector updates and it is suggested to plan your selectors up front. In any case, if you need to perform a label selector update, exercise great caution and make sure you have grasped all of the implications.

- Selector additions require the Pod template labels in the Deployment spec to be updated with the new label too, otherwise a validation error is returned. This change is a non-overlapping one, meaning that the new selector does not select ReplicaSets and Pods created with the old selector, resulting in orphaning all old ReplicaSets and creating a new ReplicaSet.
- Selector updates changes the existing value in a selector key -- result in the same behavior as additions.
- Selector removals removes an existing key from the Deployment selector -- do not require any changes in the Pod template labels. Existing ReplicaSets are not orphaned, and a new ReplicaSet is not created, but note that the removed label still exists in any existing Pods and ReplicaSets.

# Rolling back a deployment
```sh
kubectl rollout history deployment/nginx-deployment
kubectl rollout history deployment/nginx-deployment --revision=2
kubectl rollout undo deployment/nginx-deployment --to-revision=2
kubectl rollout pause deployment/nginx-deployment
kubectl rollout resume deployment/nginx-deployment
```

Max Unavailable: The maximum number of pods that can be unavailable during the update. Value can be an absolute number (for example, 5) or a percentage of desired pods (for example, 10%). This can not be 0 if Max Surge is 0. The default is 25%.

Max Surge (pico maximo): The maximum number of pods that can be scheduled above the desired number of pods. Value can be an absolute number (for example, 5) or a percentage of desired pods (for example, 10%). This can not be 0 if Max Unavailable is 0. The default is 25%.


# Set Resources
```sh
kubectl set resources deployment/nginx-deployment -c=nginx --limits=cpu=200m,memory=512Mi
```
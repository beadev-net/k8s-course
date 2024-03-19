#!bin/sh

path=$1
method="${2:-GET}"

seq 1 1 | xargs -I $ -n1 -P10  curl --request $method \
  --url http://localhost:7878/$path \
  --header 'Content-Type: application/json'
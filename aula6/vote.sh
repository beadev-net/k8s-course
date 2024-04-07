# K8S
SERVER="localhost:31150"
# Docker
#SERVER="localhost:8000"

curl -X POST $SERVER/vote -d '{"survey_id": "1", "vote": "Congrats"}' -H "Content-Type: application/json"

# LRANGE vote_queue 0 -1
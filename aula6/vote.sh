curl -X POST localhost:8000/vote -d '{"survey_id": "1", "vote": "dog"}' -H "Content-Type: application/json"

# LRANGE vote_queue 0 -1
import os
import redis
import mysql.connector
import logging
from datetime import datetime

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)

redis_db = redis.StrictRedis(host=os.getenv("REDIS_HOST", "redis"), port=6379, decode_responses=True)

db = mysql.connector.connect(
  host=os.getenv("DB_HOST", "mysql"),
  user=os.getenv("DB_USER", "user"),
  passwd=os.getenv("DB_PASSWORD", "your_password"),
  database=os.getenv("DB_DATABASE", "mydatabase"),
  port=os.getenv("DB_PORT", "3306")
)

# Função para escutar a fila e registrar os votos no MySQL
def listen():
    logger.info('Listening to vote_queue')
    while True:
        item = redis_db.brpop('vote_queue')[1]
        
        survey_id, vote = item.split(':', 1)
        
        logger.info('Received vote: %s for Survey: %s', vote, survey_id)
        
        cursor = db.cursor()
        cursor.execute("INSERT INTO votes (survey_id, vote) VALUES (%s, %s)", (survey_id, vote))

        logger.info('Vote saved in database')
        db.commit()

if __name__ == '__main__':
    listen()

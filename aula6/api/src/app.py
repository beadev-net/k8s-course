import os
import logging
import json
from datetime import datetime
import redis
import mysql.connector
from flask import Flask, request, jsonify
from flask_cors import CORS

app = Flask(__name__)
CORS(app)

redis_db = redis.StrictRedis(host=os.getenv("REDIS_HOST", "redis"), port=6379, decode_responses=True)

db = mysql.connector.connect(
    host=os.getenv("DB_HOST", "mysql"),
    user=os.getenv("DB_USER", "root"),
    passwd=os.getenv("DB_PASSWORD", "your_password"),
    database=os.getenv("DB_DATABASE", "mydatabase")
)

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)

@app.route('/healthz', methods=['GET'])
def healthcheck():
    logger.info('Healthcheck')
    return jsonify({"message": "OK"}), 200

@app.route('/vote', methods=['POST'])
def vote():
    payload = request.json
    vote = payload.get('vote')
    survey_id = payload.get('survey_id')

    logger.info('Vote received: %s for Survey: %s', vote, survey_id)

    survey_id_vote = f"{survey_id}:{vote}"
    redis_db.lpush('vote_queue', survey_id_vote)
    
    return jsonify({"message": "Vote successfully counted"}), 200

@app.route('/surveys', methods=['GET'])
def list_surveys():
    logger.info('list_surveys')

    sql = """
        SELECT * FROM surveys
    """
    cursor = db.cursor()
    cursor.execute(sql)
    result = cursor.fetchall()
    db.commit()

    if len(result) == 0:
        return jsonify({'surveys': []}), 200
    
    surveys = [{'id': row[0], 'name': row[1], 'choices': json.loads(row[2]), 'created_at': row[3]} for row in result]
    
    cursor.close()
    
    return jsonify({'surveys': surveys}), 200

@app.route('/surveys/<int:survey_id>', methods=['GET'])
def show_survey(survey_id):
    logger.info('show_survey')

    sql = """
        SELECT * FROM surveys WHERE id = %s
    """ % survey_id

    cursor = db.cursor()
    cursor.execute(sql)
    result = cursor.fetchone()
    db.commit()

    if not result:
        return jsonify({'message': 'Survey not found.'}), 404
    
    surveys = {'id': result[0], 'name': result[1], 'choices': json.loads(result[2]), 'created_at': result[3]}
    
    cursor.close()
    
    return jsonify(surveys), 200

@app.route('/surveys/<int:survey_id>/votes', methods=['GET'])
def count_votes(survey_id):
    logger.info('count_votes')

    sql = """
        SELECT vote, COUNT(*) as total FROM votes WHERE survey_id = %s GROUP BY vote; 
    """ % survey_id

    cursor = db.cursor()
    cursor.execute(sql)
    result = cursor.fetchall()
    db.commit()

    if len(result) == 0:
        return jsonify({'votes': []}), 200
    
    surveys = [{'vote': row[0], 'total': row[1]} for row in result]
    
    cursor.close()

    return jsonify({'votes': surveys, 'timestamp': datetime.now().strftime('%Y-%m-%d %H:%M:%S')}), 200

def serve():
    from waitress import serve as waitress_serve
    waitress_serve(app, host='0.0.0.0', port=5000)

if __name__ == '__main__':
    serve()
import os
from datetime import datetime
import csv
import mysql.connector

db = mysql.connector.connect(
    host=os.getenv("DB_HOST", "mysql"),
    user=os.getenv("DB_USER", "user"),
    passwd=os.getenv("DB_PASSWORD", "your_password"),
    database=os.getenv("DB_DATABASE", "mydatabase"),
    port=os.getenv("DB_PORT", "3306")
)

cursor = db.cursor()

# query = """
#   SELECT vs.survey_id, s.name, vs.vote, COUNT(*) as total
#   FROM votes vs
#   JOIN surveys s on vs.survey_id = s.id
#   GROUP BY vs.vote, s.name, vs.survey_id;
# """

query = """
  SELECT 
      vs.survey_id,
      s.name, 
      vs.vote, 
      COUNT(*) as total, 
      CASE 
          WHEN COUNT(*) = (
              SELECT MAX(total_count) 
              FROM (
                  SELECT COUNT(*) as total_count 
                  FROM votes 
                  WHERE survey_id = vs.survey_id 
                  GROUP BY vote
              ) as subquery
          ) THEN 'X'
          ELSE ''
      END as WIN
  FROM 
      votes vs
  JOIN 
      surveys s ON vs.survey_id = s.id
  GROUP BY 
      vs.vote, s.name, vs.survey_id;
"""

cursor.execute(query)

results = cursor.fetchall()

csv_file = datetime.now().strftime("%Y-%m-%d-%H-%M-%S") + ".csv"

with open(csv_file, mode='w', newline='') as file:
    writer = csv.writer(file)
    # Escreve o cabeçalho
    writer.writerow(['survey_id', 'survey', 'vote', 'total', "winner"])
    
    # Escreve cada linha de resultado
    for row in results:
        writer.writerow(row)

cursor.close()
db.close()

print(f"Arquivo CSV '{csv_file}' criado com sucesso!")

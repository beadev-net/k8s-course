import os
import logging
from datetime import datetime
import mysql.connector
from mysql.connector import Error

db = mysql.connector.connect(
    host=os.getenv("DB_HOST", "mysql"),
    user=os.getenv("DB_USER", "user"),
    passwd=os.getenv("DB_PASSWORD", "your_password"),
    database=os.getenv("DB_DATABASE", "mydatabase"),
    port=os.getenv("DB_PORT", "3306")
)

logger = logging.getLogger(__name__)
logging.basicConfig(level=logging.INFO)

def run_migration(migration_file):
    """Conecta ao banco de dados e executa o arquivo de migração SQL."""
    try:
        if db.is_connected():
            now = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            logger.info(f'{now} - MySQL Connected!')
            
            cursor = db.cursor()
            
            with open(migration_file, 'r') as file:
                migration_queries = file.read().split(';')
            
            for query in migration_queries:
                if query.strip():  # Ignorar strings vazias que podem ocorrer devido a divisões
                    print(f"Executando query: {query}")
                    cursor.execute(query)
                    db.commit()

            logger.info(f'{now} - Migration Finished Successfully!')
            
    except Error as e:
        logger.error(f'{now} - Erro ao conectar ao MySQL: {e}')
    finally:
        if (db.is_connected()):
            cursor.close()
            db.close()
            logger.info(f'{now} - MySQL connection is closed!')

# Caminho para o arquivo de migração
migration_file = 'migration.sql'

if __name__ == '__main__':
    run_migration(migration_file)

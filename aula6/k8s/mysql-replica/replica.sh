#!bin/sh
MASTER_POD='mysql-master-8-0'
REPLICA_POD='mysql-replica-8-0'
NAMESPACE='aula6'
MYSQL_ROOT_PASSWORD='your_password'
MYSQL_SOCKET='/var/lib/mysql/mysql.sock'
MASTER_SERVICE='mysql-master-service.aula6.svc.cluster.local'
REPLICA_SERVICE='mysql-replica-service.aula6.svc.cluster.local'

echo "[MASTER] Waiting the MySQL be ready..."
until mysql -h"$MASTER_SERVICE" -uroot -p"$MYSQL_ROOT_PASSWORD" --socket="$MYSQL_SOCKET" -e "SELECT 1" &> /dev/null; do
    sleep 5
done

echo "[MASTER] CREATE DUMP"
mysqldump -uroot -p"$MYSQL_ROOT_PASSWORD" -h"$MASTER_SERVICE" --socket="$MYSQL_SOCKET" --all-databases --source-data > /master_dump.sql

echo "[REPLICA] Waiting the MySQL be ready..."
until mysql -h"$REPLICA_SERVICE" -uroot -p"$MYSQL_ROOT_PASSWORD" --socket="$MYSQL_SOCKET" -e "SELECT 1" &> /dev/null; do
    sleep 5
done

echo "[REPLICA] APPLY DUMP"
mysql -uroot -p"$MYSQL_ROOT_PASSWORD" --socket="$MYSQL_SOCKET" < /master_dump.sql

REPLICA_SQL=$(cat master_dump.sql | grep "CHANGE REPLICATION SOURCE TO SOURCE_LOG_FILE=")
SOURCE_LOG_FILE=$(echo $REPLICA_SQL | awk -F"SOURCE_LOG_FILE='" '{print $2}' | awk -F"'," '{print $1; exit}')
SOURCE_LOG_POS=$(echo $REPLICA_SQL | awk -F"SOURCE_LOG_POS=" '{print $2}' | awk -F';' '{print $1; exit}')

echo "LOG_FILE: $SOURCE_LOG_FILE"
echo "LOG_POS: $SOURCE_LOG_POS"

echo "[REPLICA] CONFIGURING SOURCE"
mysql -uroot -p"$MYSQL_ROOT_PASSWORD" --socket="$MYSQL_SOCKET" -e "CHANGE SOURCE TO SOURCE_HOST='$MASTER_SERVICE', SOURCE_USER='root', SOURCE_PASSWORD='$MYSQL_ROOT_PASSWORD', SOURCE_LOG_FILE='$SOURCE_LOG_FILE', SOURCE_LOG_POS=$SOURCE_LOG_POS; START REPLICA;"

echo "[REPLICA] RUNNING"
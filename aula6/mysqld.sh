#!bin/sh

./wait-for-it.sh localhost:3306 -- echo "mysql-master is up"

docker-compose exec mysql-master mysqldump -u root -p'your_password' --all-databases --master-data > master_dump.sql

docker-compose cp master_dump.sql mysql-replica:/master_dump.sql

docker-compose exec -T mysql-replica mysql -u root -p'your_password' < master_dump.sql

REPLICA_SQL=$(cat master_dump.sql | grep "CHANGE REPLICATION SOURCE TO SOURCE_LOG_FILE=")
SOURCE_LOG_FILE=$(echo $REPLICA_SQL | awk -F"SOURCE_LOG_FILE='" '{print $2}' | awk -F"'," '{print $1; exit}')
SOURCE_LOG_POS=$(echo $REPLICA_SQL | awk -F"SOURCE_LOG_POS=" '{print $2}' | awk -F';' '{print $1; exit}')

echo "LOG_FILE: $SOURCE_LOG_FILE"
echo "LOG_POS: $SOURCE_LOG_POS"

docker-compose exec -T mysql-replica mysql -u root -p'your_password' -e "CHANGE MASTER TO MASTER_HOST='mysql-master', MASTER_USER='root', MASTER_PASSWORD='your_password', MASTER_LOG_FILE='$SOURCE_LOG_FILE', MASTER_LOG_POS=$SOURCE_LOG_POS;";

echo "START REPLICA"

docker-compose exec -T mysql-replica mysql -u root -p'your_password' -e 'START REPLICA';

echo "REPLICA RUNNING"
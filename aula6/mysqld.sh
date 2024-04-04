mysqldump -u root -p --all-databases --master-data > master_dump.sql

docker-compose exec mysql mysqldump -u root -p'your_password' --all-databases --master-data > master_dump.sql

docker-compose cp master_dump.sql mysql-slave:/master_dump.sql

docker-compose exec -T mysql-slave mysql -u root -p'your_password' < master_dump.sql

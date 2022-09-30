CREATE USER IF NOT EXISTS '$MYSQL_USER' IDENTIFIED BY '$MYSQL_PASSWORD';

DROP DATABASE IF EXISTS $MYSQL_DATABASE;

CREATE DATABASE $MYSQL_DATABASE;

GRANT ALL ON $MYSQL_DATABASE.* TO '$MYSQL_USER';
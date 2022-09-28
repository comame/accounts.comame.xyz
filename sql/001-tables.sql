USE $MYSQL_DATABASE;

CREATE TABLE users (id VARCHAR(100) PRIMARY KEY);

CREATE TABLE user_passwords (user_id TEXT, hashed_password TEXT);

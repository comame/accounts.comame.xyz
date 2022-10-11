USE $MYSQL_DATABASE;

CREATE TABLE users (
    id VARCHAR(100) PRIMARY KEY
);

CREATE TABLE user_passwords (
    user_id VARCHAR(100) PRIMARY KEY,
    hashed_password TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE `sessions` (
    user_id VARCHAR(100) NOT NULL,
    token VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE authentications (
    authenticated_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    audience VARCHAR(100) NOT NULL,
    `subject` VARCHAR(100) NOT NULL,
    method VARCHAR(16) NOT NULL,
    prompt VARCHAR(16) NOT NULL
);

CREATE TABLE authentication_failures (
    tried_at TIMESTAMP NOT NULL,
    audience VARCHAR(100) NOT NULL,
    `subject` VARCHAR(100) NOT NULL,
    method VARCHAR(16) NOT NULL,
    reason VARCHAR(32) NOT NULL
);

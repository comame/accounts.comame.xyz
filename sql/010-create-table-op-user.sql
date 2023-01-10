CREATE TABLE op_user (
    user_id VARCHAR(100) NOT NULL,
    op_user_id VARCHAR(100) NOT NULL,
    op VARCHAR(16) NOT NULL,
    UNIQUE (op_user_id, op),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

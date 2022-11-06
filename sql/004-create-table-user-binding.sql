CREATE TABLE user_bindings (
    relying_party_id VARCHAR(100) NOT NULL,
    user_id VARCHAR(100) NOT NULL,
    UNIQUE (relying_party_id, user_id),
    FOREIGN KEY (relying_party_id) REFERENCES relying_parties(client_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (relying_party_id, user_id)
);

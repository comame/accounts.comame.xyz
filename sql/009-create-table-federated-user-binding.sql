CREATE TABLE federated_user_binding (
    relying_party_id VARCHAR(100) NOT NULL,
    issuer VARCHAR(100) NOT NULL,
    UNIQUE (relying_party_id, issuer),
    FOREIGN KEY (relying_party_id) REFERENCES relying_parties(client_id) ON DELETE CASCADE,
    PRIMARY KEY (relying_party_id, issuer)
);

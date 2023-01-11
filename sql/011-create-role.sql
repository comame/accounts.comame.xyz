DROP TABLE `federated_user_binding`;
DROP TABLE `user_bindings`;

CREATE TABLE role (
    `name` VARCHAR(100) NOT NULL PRIMARY KEY
);

CREATE TABLE user_role (
    user_id VARCHAR(100) NOT NULL,
    `role` VARCHAR(100) NOT NULL,
    UNIQUE (user_id, `role`),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (`role`) REFERENCES `role`(`name`) ON DELETE CASCADE
);

CREATE TABLE role_access (
    `role` VARCHAR(100) NOT NULL,
    relying_party_id VARCHAR(100) NOT NULL,
    UNIQUE (`role`, relying_party_id),
    FOREIGN KEY (`role`) REFERENCES `role`(`name`) ON DELETE CASCADE,
    FOREIGN KEY (relying_party_id) REFERENCES relying_parties(client_id) ON DELETE CASCADE

);

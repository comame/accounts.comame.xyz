CREATE TABLE access_tokens (
    sub VARCHAR(100) NOT NULL,
    scopes TEXT NOT NULL,
    token VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (`sub`) REFERENCES `users` (`id`) ON DELETE CASCADE
)

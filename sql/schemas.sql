access_tokens	CREATE TABLE `access_tokens` (
  `sub` varchar(100) NOT NULL,
  `scopes` text NOT NULL,
  `token` varchar(100) NOT NULL,
  `created_at` timestamp NOT NULL,
  KEY `sub` (`sub`),
  CONSTRAINT `access_tokens_ibfk_1` FOREIGN KEY (`sub`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

authentication_failures	CREATE TABLE `authentication_failures` (
  `tried_at` timestamp NOT NULL,
  `subject` varchar(100) NOT NULL,
  `method` varchar(16) NOT NULL,
  `reason` varchar(32) NOT NULL,
  `clean` tinyint(1) NOT NULL DEFAULT '0',
  `remote_addr` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

authentications	CREATE TABLE `authentications` (
  `authenticated_at` timestamp NOT NULL,
  `created_at` timestamp NOT NULL,
  `audience` varchar(100) NOT NULL,
  `subject` varchar(100) NOT NULL,
  `user_agent_id` varchar(100) NOT NULL,
  `method` varchar(16) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

idtoken_issues	CREATE TABLE `idtoken_issues` (
  `sub` varchar(100) NOT NULL,
  `aud` varchar(100) NOT NULL,
  `iat` timestamp NOT NULL,
  `remote_addr` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

mails	CREATE TABLE `mails` (
  `subject` text NOT NULL,
  `to` text NOT NULL,
  `body` text NOT NULL,
  `created_at` timestamp NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

redirect_uris	CREATE TABLE `redirect_uris` (
  `client_id` varchar(100) NOT NULL,
  `redirect_uri` varchar(100) NOT NULL,
  UNIQUE KEY `client_id` (`client_id`,`redirect_uri`),
  CONSTRAINT `redirect_uris_ibfk_1` FOREIGN KEY (`client_id`) REFERENCES `relying_parties` (`client_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

relying_parties	CREATE TABLE `relying_parties` (
  `client_id` varchar(100) NOT NULL,
  `hashed_client_secret` text NOT NULL,
  PRIMARY KEY (`client_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

rsa_keypair	CREATE TABLE `rsa_keypair` (
  `id` enum('1') NOT NULL DEFAULT '1',
  `public` text NOT NULL,
  `private` text NOT NULL,
  `kid` varchar(8) NOT NULL,
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

sessions	CREATE TABLE `sessions` (
  `user_id` varchar(100) NOT NULL,
  `token` varchar(128) NOT NULL,
  `created_at` timestamp NOT NULL,
  UNIQUE KEY `token` (`token`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `sessions_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

user_bindings	CREATE TABLE `user_bindings` (
  `relying_party_id` varchar(100) NOT NULL,
  `user_id` varchar(100) NOT NULL,
  PRIMARY KEY (`relying_party_id`,`user_id`),
  UNIQUE KEY `relying_party_id` (`relying_party_id`,`user_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `user_bindings_ibfk_1` FOREIGN KEY (`relying_party_id`) REFERENCES `relying_parties` (`client_id`) ON DELETE CASCADE,
  CONSTRAINT `user_bindings_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

user_passwords	CREATE TABLE `user_passwords` (
  `user_id` varchar(100) NOT NULL,
  `hashed_password` text NOT NULL,
  PRIMARY KEY (`user_id`),
  CONSTRAINT `user_passwords_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

users	CREATE TABLE `users` (
  `id` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


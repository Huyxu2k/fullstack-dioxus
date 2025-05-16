CREATE TABLE `users` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `employee_id` INT,
  `username` VARCHAR(50) UNIQUE NOT NULL,
  `password_hash` VARCHAR(255) NOT NULL,
  `email` VARCHAR(255) UNIQUE NOT NULL,
  `is_active` BOOLEAN DEFAULT true,
  `created_at` DATETIME DEFAULT (CURRENT_TIMESTAMP)
);

CREATE TABLE `roles` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `name` VARCHAR(50) UNIQUE NOT NULL,
  `description` TEXT
);

CREATE TABLE `permissions` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `resource` VARCHAR(50) NOT NULL,
  `action` VARCHAR(50) NOT NULL,
  `description` TEXT
);

CREATE TABLE `actions` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `key` VARCHAR(50) NOT NULL,
  `description` TEXT
);

CREATE TABLE `user_roles` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `user_id` INT NOT NULL,
  `role_id` INT NOT NULL
);

CREATE TABLE `role_permissions` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `role_id` INT NOT NULL,
  `permission_id` INT NOT NULL
);

CREATE TABLE `tokens` (
  `id` INT PRIMARY KEY AUTO_INCREMENT,
  `user_id` INT NOT NULL,
  `token` VARCHAR(255) UNIQUE NOT NULL,
  `device_id` VARCHAR(50),
  `device_info` TEXT,
  `ip_address` VARCHAR(45),
  `created_at` DATETIME DEFAULT (CURRENT_TIMESTAMP),
  `last_used_at` DATETIME,
  `expires_at` DATETIME NOT NULL,
  `revoked` BOOLEAN DEFAULT false
);

-- ALTER TABLE `user_roles` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);

-- ALTER TABLE `user_roles` ADD FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`);

-- ALTER TABLE `role_permissions` ADD FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`);

-- ALTER TABLE `role_permissions` ADD FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`);

-- ALTER TABLE `tokens` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);

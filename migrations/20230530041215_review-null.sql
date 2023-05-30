-- Add migration script here
ALTER TABLE `mob`.`review`
CHANGE COLUMN `user_id` `user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `text` `text` VARCHAR(450) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
CHANGE COLUMN `stars` `stars` TINYINT UNSIGNED NOT NULL ,
CHANGE COLUMN `location_name` `location_name` VARCHAR(96) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
CHANGE COLUMN `is_custom` `is_custom` TINYINT(1) NOT NULL ,
CHANGE COLUMN `category` `category` VARCHAR(30) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
ADD COLUMN `delivered` TINYINT NOT NULL AFTER `location` ;

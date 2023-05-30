-- Add migration script here
ALTER TABLE `mob`.`reply`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `user_id` `user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `text` `text` VARCHAR(450) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
CHANGE COLUMN `review_id` `review_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `reply_to_id` `reply_to_id` VARCHAR(36) NULL ;

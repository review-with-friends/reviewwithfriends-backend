-- Add migration script here
ALTER TABLE `mob`.`notification`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `review_user_id` `review_user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `review_id` `review_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `user_id` `user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `action_type` `action_type` TINYINT UNSIGNED NOT NULL ;

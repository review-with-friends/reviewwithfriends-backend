-- Add migration script here
ALTER TABLE `mob`.`friend`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `user_id` `user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `friend_id` `friend_id` VARCHAR(36) NOT NULL ;

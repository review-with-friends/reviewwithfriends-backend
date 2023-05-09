-- Add migration script here
ALTER TABLE `mob`.`reports`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `user_id` `user_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `reporter_id` `reporter_id` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `report_type` `report_type` TINYINT UNSIGNED NOT NULL ;

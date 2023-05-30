-- Add migration script here
ALTER TABLE `mob`.`authattempt`
CHANGE COLUMN `phone` `phone` VARCHAR(25) NOT NULL ,
CHANGE COLUMN `created` `created` DATETIME NOT NULL ;

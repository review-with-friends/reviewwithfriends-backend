-- Add migration script here
ALTER TABLE `mob`.`phoneauth`
CHANGE COLUMN `phone` `phone` VARCHAR(25) NOT NULL ,
CHANGE COLUMN `code` `code` VARCHAR(9) NOT NULL ,
CHANGE COLUMN `ip` `ip` VARCHAR(36) NOT NULL ,
CHANGE COLUMN `used` `used` TINYINT(1) NOT NULL ;

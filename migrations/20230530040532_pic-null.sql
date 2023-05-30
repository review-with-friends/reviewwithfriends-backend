-- Add migration script here
ALTER TABLE `mob`.`pic`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `pic_handler` `pic_handler` TINYINT UNSIGNED NOT NULL ,
CHANGE COLUMN `width` `width` SMALLINT UNSIGNED NOT NULL ,
CHANGE COLUMN `height` `height` SMALLINT UNSIGNED NOT NULL ;

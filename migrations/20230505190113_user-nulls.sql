ALTER TABLE `mob`.`user`
CHANGE COLUMN `created` `created` DATETIME NOT NULL ,
CHANGE COLUMN `name` `name` VARCHAR(26) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
CHANGE COLUMN `display_name` `display_name` VARCHAR(26) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL ,
CHANGE COLUMN `phone` `phone` VARCHAR(25) NOT NULL ,
CHANGE COLUMN `pic_id` `pic_id` VARCHAR(36) NOT NULL ;

-- Add migration script here
ALTER TABLE `mob`.`review`
ADD COLUMN `recommended` TINYINT NOT NULL DEFAULT 0 AFTER `delivered`;

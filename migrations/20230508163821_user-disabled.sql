-- Add migration script here
ALTER TABLE `mob`.`user`
ADD COLUMN `disabled` BOOLEAN NOT NULL DEFAULT false AFTER `email`;

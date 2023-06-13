-- Add migration script here
ALTER TABLE `mob`.`likes`
ADD COLUMN `like_type` TINYINT(8) NOT NULL DEFAULT 0 AFTER `review_id`;

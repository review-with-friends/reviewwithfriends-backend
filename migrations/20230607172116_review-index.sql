-- Add migration script here
ALTER TABLE `mob`.`review`
ADD INDEX `idx_review_user_id_recommended` (`user_id` ASC, `recommended` ASC) VISIBLE;

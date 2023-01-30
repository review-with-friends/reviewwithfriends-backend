CREATE TABLE notification (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    review_user_id VARCHAR(36),
    review_id varchar(36),
    user_id VARCHAR(36),
    action_type TINYINT UNSIGNED
);
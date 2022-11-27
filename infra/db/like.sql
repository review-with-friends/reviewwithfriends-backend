CREATE TABLE likes (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    user_id VARCHAR(36),
    review_id VARCHAR(36)
);
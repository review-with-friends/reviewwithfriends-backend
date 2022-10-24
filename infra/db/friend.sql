CREATE TABLE friend (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    user_id VARCHAR(36),
    friend_id VARCHAR(36)
);
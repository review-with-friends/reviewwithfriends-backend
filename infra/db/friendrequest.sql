CREATE TABLE friendrequest (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    user_id VARCHAR(36),
    friend_id VARCHAR(36),
    ignored BOOLEAN
);
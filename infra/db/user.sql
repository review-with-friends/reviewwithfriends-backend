CREATE TABLE user (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    name VARCHAR(255),
    display_name VARCHAR(255),
    phone VARCHAR(25)
);
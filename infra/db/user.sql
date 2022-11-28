CREATE TABLE user (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    name VARCHAR(30),
    display_name VARCHAR(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin,
    phone VARCHAR(25)
);
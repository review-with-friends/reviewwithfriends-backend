CREATE TABLE user (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    name VARCHAR(255),
    display_name VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin,
    phone VARCHAR(25)
);
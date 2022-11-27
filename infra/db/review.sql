CREATE TABLE review (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    user_id VARCHAR(36),
    created DATETIME,
    pic_id VARCHAR(36) NULL,
    category VARCHAR(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci,
    text VARCHAR(450) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci,
    stars TINYINT UNSIGNED,
    location_name VARCHAR(96) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci,
    latitude DOUBLE,
    longitude DOUBLE,
    is_custom BOOLEAN
);
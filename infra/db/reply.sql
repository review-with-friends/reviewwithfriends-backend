CREATE TABLE reply (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    user_id VARCHAR(36),
    review_id varchar(36),
    text VARCHAR(450) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci,
    reply_to_id VARCHAR(36) NULL,
);

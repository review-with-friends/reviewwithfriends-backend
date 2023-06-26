-- Add migration script here
CREATE TABLE bookmark (
  id varchar(36) NOT NULL,
  user_id varchar(36) NOT NULL,
  created datetime NOT NULL,
  category varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  location_name varchar(96) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  location point NOT NULL /*!80003 SRID 0 */,
  PRIMARY KEY (id),
  KEY idx_review_location_name (location_name),
  KEY idx_review_user_id (user_id),
  KEY idx_review_category (category),
  KEY idx_review_created (created),
  SPATIAL KEY idx_review_location (location)
);

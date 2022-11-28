# Core Values

## 5XX Retry && 4XX Terminal

Never assume data will exist. Always have a plan to return an appropriate response if it doesn't exist. We assume clients are able to freely retry 5XX errors, and can be confident 4XX should not be automatically retries unless user input is engaged.

## 4XX be User Surfacable

As a user, knowing what went wrong with my request is helpful in understanding my issue. Often shitty apps never consider failure cases, and we embrace failure cases. Any user interaction needs to have the ability to surface 4XX error message to enable the user to correct their mistake or retry.

# Database Types

## Ping

| Column | Type                   |
| ------ | ---------------------- |
| id     | VARCHAR(36)PRIMARY KEY |

## User

| Column       | Type                   |
| ------------ | ---------------------- |
| id           | VARCHAR(36)PRIMARY KEY |
| created      | DATETIME               |
| name         | VARCHAR(30)            |
| display_name | VARCHAR(30)            |
| phone        | VARCHAR(25)            |

## PhoneAuth

| Column  | Type                    |
| ------- | ----------------------- |
| id      | VARCHAR(36) PRIMARY KEY |
| phone   | VARCHAR(25)             |
| created | DATETIME                |
| code    | VARCHAR(9)              |
| ip      | VARCHAR(36)             |
| used    | BOOLEAN                 |

## AuthAttempt

| Column  | Type                    |
| ------- | ----------------------- |
| id      | VARCHAR(36) PRIMARY KEY |
| phone   | VARCHAR(25)             |
| created | DATETIME                |

## Friend

| Column    | Type                    |
| --------- | ----------------------- |
| id        | VARCHAR(36) PRIMARY KEY |
| created   | DATETIME                |
| user_id   | VARCHAR(36)             |
| friend_id | VARCHAR(36)             |

## Friend Request

| Column    | Type                    |
| --------- | ----------------------- |
| id        | VARCHAR(36) PRIMARY KEY |
| created   | DATETIME                |
| user_id   | VARCHAR(36)             |
| friend_id | VARCHAR(36)             |
| ignored   | BOOLEAN                 |

## Pic

| Column      | Type                    |
| ----------- | ----------------------- |
| id          | VARCHAR(36) PRIMARY KEY |
| created     | DATETIME                |
| pic_handler | TINYINT UNSIGNED        |

## Review

| Column        | Type                    |
| ------------- | ----------------------- |
| id            | VARCHAR(36) PRIMARY KEY |
| user_id       | VARCHAR(36)             |
| created       | DATETIME                |
| pic_id        | VARCHAR(36) NULL        |
| category      | VARCHAR(30)             |
| text          | VARCHAR(450) utf8mb4    |
| stars         | TINYINT UNSIGNED        |
| location_name | VARCHAR(96) utf8mb4     |
| latitude      | DOUBLE                  |
| longitude     | DOUBLE                  |
| is_custom     | BOOLEAN                 |

## Like

| Column    | Type                    |
| --------- | ----------------------- |
| id        | VARCHAR(36) PRIMARY KEY |
| created   | DATETIME                |
| user_id   | VARCHAR(36)             |
| review_id | VARCHAR(36)             |

## Reply

| Column    | Type                    |
| --------- | ----------------------- |
| id        | VARCHAR(36) PRIMARY KEY |
| created   | DATETIME                |
| user_id   | VARCHAR(36)             |
| review_id | VARCHAR(36)             |
| text      | VARCHAR(450) utf8mb4    |

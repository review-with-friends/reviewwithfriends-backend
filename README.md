[![Release](https://github.com/review-with-friends/reviewwithfriends-backend/actions/workflows/rust.yml/badge.svg)](https://github.com/review-with-friends/reviewwithfriends-backend/actions/workflows/rust.yml)

# Core Values

## 5XX Retry && 4XX Terminal

Never assume data will exist. Always have a plan to return an appropriate response if it doesn't exist. We assume clients are able to freely retry 5XX errors, and can be confident 4XX should not be automatically retried unless user input is engaged.

## 4XX body be User Surfacable

As a user, knowing what went wrong with my request is helpful in understanding my issue. Often bad apps never consider failure cases, and we want to embrace failure cases. Any user interaction needs to have the ability to surface 4XX error message to enable the user to correct their mistake or retry.

## Remember Mobile Connections Sometimes Suck

As a user, I could be communicating with your backend from a train or as a passenger in a car. This can lead to tons of variance in the success of individual requests.

We want to minimize how many requests we send. If possible, we'd like to return compound types containing data most commonly needed in a single request. An example of this is a review. When I want to fetch a review, initial comments and the likes on it are almost always required with that.

## Be Data Conscious

Most users in the world have limited data plans. We can't expect people to always load large amounts of data. If possible, make sure consumers can request specific information and only in chunks at a time.

We don't want to load 100's of reviews that I can't possibly see yet without vigorous scrolling. Allow clients to request chunks at a time, for a quicker response when connections are slower.

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

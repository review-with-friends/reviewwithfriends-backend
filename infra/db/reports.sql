CREATE TABLE reports (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    created DATETIME,
    user_id VARCHAR(36),
    reporter_id VARCHAR(36),
    report_type TINYINT UNSIGNED
);

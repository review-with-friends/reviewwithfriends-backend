CREATE TABLE pic (  
    id varchar(36) NOT NULL PRIMARY KEY,
    review_id varchar(36),
    created DATETIME,
    pic TINYINT UNSIGNED,
    width SMALLINT UNSIGNED,
    height SMALLINT UNSIGNED
);
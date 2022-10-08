CREATE TABLE phoneauth (  
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    phone VARCHAR(25),
    created DATETIME,
    code VARCHAR(9),
    ip VARCHAR(36),
    used BOOLEAN
);
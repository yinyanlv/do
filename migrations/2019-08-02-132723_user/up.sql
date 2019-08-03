-- Your SQL goes here

CREATE TABLE user (
    id VARCHAR(40) NOT NULL PRIMARY KEY,
    username VARCHAR(20) NOT NULL,
    nickname VARCHAR(20) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    email VARCHAR(40) NOT NULL,
    password VARCHAR(40) NOT NULL,
    UNIQUE KEY username (username)
) ENGINE = InnoDB DEFAULT CHARSET = utf8;
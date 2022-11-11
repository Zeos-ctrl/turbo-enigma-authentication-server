CREATE TABLE IF NOT EXISTS db (
    uuid varchar(255) NOT NULL,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    phonenumber INT,
    seconds INT,
    CONSTRAINT db UNIQUE (username,email)
);

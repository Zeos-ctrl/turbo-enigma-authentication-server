CREATE DATABASE IF NOT EXISTS db;

USE db;

CREATE TABLE IF NOT EXISTS users (
    uuid varchar(255) NOT NULL,
    username varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    phonenumber INT,
    secret varchar(255),
    CONSTRAINT users UNIQUE (username,email)
);

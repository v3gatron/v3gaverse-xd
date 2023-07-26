-- Add up migration script here

create table if not exists architects
(id SERIAL NOT NULL PRIMARY KEY,
 handle varchar(30) NOT NULL UNIQUE,
 password varchar(100) NOT NULL,         
 created_at TIMESTAMP DEFAULT Now())
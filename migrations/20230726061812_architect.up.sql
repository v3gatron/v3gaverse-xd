-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists architects
(id UUID NOT NULL PRIMARY KEY,
 handle varchar(30) NOT NULL UNIQUE,
 password varchar(100) NOT NULL,         
 created_at TIMESTAMP DEFAULT Now(),
 last_login TIMESTAMP);

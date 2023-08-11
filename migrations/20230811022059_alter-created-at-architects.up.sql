-- Add up migration script here
ALTER TABLE architects
ADD updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp;

-- Add up migration script here
CREATE TABLE document(
    id SERIAL PRIMARY KEY,
    owner_id int NOT NULL REFERENCES patient(id),
    name varchar(50),
    content BYTEA,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

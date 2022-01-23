-- Your SQL goes here
CREATE TABLE greetings (id SERIAL PRIMARY KEY, greeting VARCHAR NOT NULL);
INSERT INTO greetings (greeting) VALUES ('Hello world');

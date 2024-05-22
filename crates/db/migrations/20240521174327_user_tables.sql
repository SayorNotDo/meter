-- migrate:up

CREATE TABLE users
(
	id 					SERIAL PRIMARY KEY,
	uuid 				UUID 	NOT NULL,
	username 			VARCHAR NOT NULL UNIQUE,
	hashed_password 	VARCHAR NOT NULL,
    email 				VARCHAR UNIQUE,
    created_at 			TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at			TIMESTAMP
);

-- trigger function: update current_timestamp
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
	NEW.updated_at = NOW();
  	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


-- migrate:down
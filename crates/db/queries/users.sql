--: User()
--! get_users : User
SELECT 
	id,
   	username,
   	email,
   	created_at,
   	updated_at
FROM users;

--! insert_user
INSERT INTO 
	users (username, hashed_password, email, uuid)
VALUES
	(:username, :hashed_password, :email, :uuid)
RETURNING id;

--! get_user_by_username (username?) : (email?, hashed_password?, updated_at?)
SELECT 
	id,
	username,
	hashed_password,
	email,
	created_at,
	updated_at
FROM users
WHERE username = :username;

--! get_user_by_email (email?)
SELECT
    id,
    username,
    email,
    created_at
FROM users
WHERE email = :email;
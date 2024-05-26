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
	users (username, hashed_password, uuid)
VALUES 
	(:username, :hashed_password, :uuid);

--! get_user_by_username
SELECT 
	id,
   	username,
   	email,
   	created_at,
   	updated_at
FROM users
WHERE username = :username;
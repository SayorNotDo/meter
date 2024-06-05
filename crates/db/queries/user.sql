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

--! get_user_by_username (username?) : (updated_at?, last_organization_id?, last_project_id?)
SELECT 
	id,
	uuid,
	username,
	hashed_password,
	email,
	created_at,
	updated_at,
    last_organization_id,
    last_project_id
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

--! get_user_by_uuid (uuid?) : (updated_at?, last_organization_id?, last_project_id?)
SELECT
    id,
    uuid,
    username,
    hashed_password,
    email,
    created_at,
    updated_at,
    last_organization_id,
    last_project_id
FROM users
WHERE uuid = :uuid;

--！get_user_role_by_uuid (uuid?) : (updated_at?, last_organization_id?, last_project_id?)
SELECT
    id,
    name,
    type,
    description,
    created_at
FROM user_role
WHERE uuid = :uuid;
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

--! get_user_roles_by_uuid
SELECT
    r.name,
    r.created_at,
    r.created_by,
    r.updated_at,
    r.description,
    r.type
    FROM users u
    JOIN user_role_relation urr ON u.uuid = urr.user_id
    JOIN user_role r ON urr.role_id = r.id
WHERE u.uuid = :uuid;
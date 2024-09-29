--! get_users : (updated_at?, last_organization_id?, last_project_id?)
SELECT id,
       uuid,
       username,
       hashed_password,
       email,
       created_at,
       updated_at,
       last_organization_id,
       last_project_id
FROM users;

--! insert_user
INSERT INTO users (username, hashed_password, email, uuid)
VALUES (:username, :hashed_password, :email, :uuid)
RETURNING id;

--! get_user_by_username (username?) : (updated_at?, last_organization_id?, last_project_id?)
SELECT id,
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
SELECT id,
       username,
       email,
       created_at
FROM users
WHERE email = :email;

--! get_user_by_uuid : (updated_at?, last_organization_id?, last_project_id?)
SELECT id,
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

--! get_user_roles_by_uuid : (updated_at?, description?)
SELECT r.id,
       r.name,
       r.created_at,
       r.internal,
       creator.username as created_by,
       r.updated_at,
       r.description,
       r.type           as role_type
FROM users u
         JOIN user_role_relation urr ON u.uuid = urr.user_id
         JOIN user_role r ON urr.role_id = r.id
         JOIN users creator ON creator.uuid = r.created_by
WHERE u.uuid = :uuid;

--! get_user_role_list_by_project_id
SELECT urr.role_id as id,
       ur.name
FROM user_role_relation urr
LEFT JOIN user_role ur ON urr.role_id = ur.id
WHERE urr.project_id = :project_id;

--! get_user_role_relations_by_uuid
SELECT urr.id,
       urr.created_at,
       urr.user_id,
       urr.role_id,
       creator.username as created_by,
       urr.organization_id
FROM users u
         JOIN user_role_relation urr ON u.uuid = urr.user_id
         JOIN users creator ON creator.uuid = urr.created_by
WHERE u.uuid = :uuid;

--! get_user_role_permissions_by_role_id
SELECT id,
       role_id,
       permission
FROM user_role_permission
WHERE role_id = :role_id;

--! get_users_by_role_and_project_id : (updated_at?, last_organization_id?, last_project_id?)
SELECT u.id,
       u.uuid,
       u.username,
       u.hashed_password,
       u.email,
       u.created_at,
       u.updated_at,
       u.last_organization_id,
       u.last_project_id
FROM users u
JOIN user_role_relation urr ON u.uuid = urr.user_id
JOIN projects p ON urr.organization_id = p.organization_id
JOIN user_role ur ON urr.role_id = ur.id
WHERE p.id = :project_id AND ur.name = :role_name;

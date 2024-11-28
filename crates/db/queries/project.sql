--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       CAST((SELECT COUNT(*)
             FROM users
                      LEFT JOIN user_role_relation urr ON urr.project_id = p.id) AS INTEGER) AS member_count,
       p.created_at,
       uc.username                                                                                             AS created_by,
       p.updated_at,
       uu.username                                                                                             AS updated_by,
       ud.username                                                                                             AS deleted_by,
       p.deleted_at,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
         LEFT JOIN users uc ON p.created_by = uc.uuid
         LEFT JOIN users uu ON p.updated_by = uu.uuid
         LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.id = :id;

--! find_project_by_name : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       CAST((SELECT COUNT(*)
             FROM users
                      LEFT JOIN user_role_relation urr ON urr.project_id = p.id) AS INTEGER) AS member_count,
       p.created_at,
       uc.username                                                                                             AS created_by,
       p.updated_at,
       uu.username                                                                                             AS updated_by,
       ud.username                                                                                             AS deleted_by,
       p.deleted_at,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
         LEFT JOIN users uc ON p.created_by = uc.uuid
         LEFT JOIN users uu ON p.updated_by = uu.uuid
         LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.name = :name;

--! find_projects_by_uid : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       p.created_at,
       uc.username                                                         AS created_by,
       p.updated_at,
       uu.username                                                         AS updated_by,
       ud.username                                                         AS deleted_by,
       CAST((SELECT COUNT(*)
             FROM users
                      LEFT JOIN
                  user_role_relation urr ON
                      urr.project_id = p.id) AS INTEGER) AS member_count,
       p.deleted_at,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
         LEFT JOIN users uc ON p.created_by = uc.uuid
         LEFT JOIN users uu ON p.updated_by = uu.uuid
         LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.created_by = :uid;

--! get_projects_by_uid : (updated_at?, updated_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       p.created_at,
       uc.username                                                         AS created_by,
       p.updated_at,
       uu.username                                                         AS updated_by,
       CAST(COUNT(urr.user_id) AS INTEGER) AS member_count,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
    LEFT JOIN users uc ON p.created_by = uc.uuid
    LEFT JOIN users uu ON p.updated_by = uu.uuid
    LEFT JOIN user_role_relation urr ON urr.project_id = p.id
WHERE urr.user_id = :uid
GROUP BY p.id, p.name, uc.username, uu.username;


--! get_project_members : (last_project_id?)
SELECT
    u.id,
    u.username,
    u.email,
    u.created_at,
    u.last_project_id
FROM users u
    INNER JOIN user_role_relation urr ON urr.user_id = u.uuid
    INNER JOIN projects p ON urr.project_id = p.id
WHERE p.id = :project_id;


--! insert_project
INSERT INTO projects (name,
                      created_by,
                      description,
                      module_setting)
VALUES (:name,
        :created_by,
        :description,
        :module_setting) RETURNING id;

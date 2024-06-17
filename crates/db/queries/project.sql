--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       o.name                                                                                                  AS organization,
       CAST((SELECT COUNT(*)
             FROM users
                      LEFT JOIN user_role_relation urr ON urr.organization_id = p.organization_id) AS INTEGER) AS member_count,
       p.created_at,
       uc.username                                                                                             AS created_by,
       p.updated_at,
       uu.username                                                                                             AS updated_by,
       ud.username                                                                                             AS deleted_by,
       p.deleted,
       p.deleted_at,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
         LEFT JOIN organizations o ON p.organization_id = o.id
         LEFT JOIN users uc ON p.created_by = uc.uuid
         LEFT JOIN users uu ON p.updated_by = uu.uuid
         LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.id = :id;

--! find_projects_by_uid : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       o.name                                                                AS organization,
       p.created_at,
       uc.username                                                         AS created_by,
       p.updated_at,
       uu.username                                                         AS updated_by,
       ud.username                                                         AS deleted_by,
       CAST((SELECT COUNT(*)
             FROM users
                      LEFT JOIN
                  user_role_relation urr ON
                      urr.organization_id = p.organization_id) AS INTEGER) AS member_count,
       p.deleted,
       p.deleted_at,
       p.enable,
       p.description,
       p.module_setting
FROM projects p
         LEFT JOIN organizations o ON p.organization_id = o.id
         LEFT JOIN users uc ON p.created_by = uc.uuid
         LEFT JOIN users uu ON p.updated_by = uu.uuid
         LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.created_by = :uid
  AND p.organization_id = :organization_id;


--! get_project_members : (last_project_id?, last_organization_id?)
SELECT
    u.id,
    u.username,
    u.email,
    u.created_at,
    u.last_project_id,
    u.last_organization_id
FROM users u
    INNER JOIN user_role_relation urr ON urr.user_id = u.uuid
    INNER JOIN projects p ON urr.organization_id = p.organization_id
WHERE p.id = :project_id;


--! insert_project
INSERT INTO projects (name,
                      organization_id,
                      created_by,
                      description,
                      module_setting)
VALUES (:name,
        :organization_id,
        :created_by,
        :description,
        :module_setting) RETURNING id;

--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT id,
       name,
       organization_id,
       created_at,
       created_by,
       updated_at,
       updated_by,
       deleted_by,
       deleted_at,
       description,
       module_setting
FROM projects
WHERE id = :id;

--! find_projects_by_uid : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT projects.id,
       projects.name,
       projects.created_at,
       uc.username                                               as created_by,
       projects.updated_at,
       uu.username                                               as updated_by,
       ud.username                                               as deleted_by,
       (SELECT COUNT(*)
        FROM users
                 LEFT JOIN
             user_role_relation urr ON
                 urr.organization_id = projects.organization_id) as member_count,
       deleted,
       deleted_at,
       description,
       module_setting
FROM projects
         LEFT JOIN users uc ON projects.created_by = uc.uuid
         LEFT JOIN users uu ON projects.updated_by = uu.uuid
         LEFT JOIN users ud ON projects.deleted_by = ud.uuid
WHERE created_by = :uid;

--! insert_project
INSERT INTO projects (name, organization_id, created_by, description, module_setting)
VALUES (:name, :organization_id, :created_by, :description, :module_setting) RETURNING id;
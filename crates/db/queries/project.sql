--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT p.id,
       p.name,
       o.name as organization,
       p.created_at,
       uc.username as created_by,
       p.updated_at,
       uu.username as updated_by,
       ud.username as deleted_by,
       p.deleted_at,
       p.description,
       p.module_setting
FROM projects p
LEFT JOIN organizations o ON p.organization_id = o.id
LEFT JOIN users uc ON p.created_by = uc.uuid
LEFT JOIN users uu ON p.updated_by = uu.uuid
LEFT JOIN users ud ON p.deleted_by = ud.uuid
WHERE p.id = :id;

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
INSERT INTO
projects (name,
 organization_id,
  created_by,
  description,
   module_setting)
VALUES (
:name,
:organization_id,
:created_by,
:description,
:module_setting)
RETURNING id;
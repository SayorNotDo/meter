--! get_file_modules : (parent_id?)
WITH RECURSIVE file_module_tree AS (SELECT id,
                                           name,
                                           position,
                                           module_type,
                                           parent_id
                                    FROM file_module
                                    WHERE project_id = :project_id
                                    AND module_type = :module_type
                                    UNION ALL
                                    SELECT f.id,
                                           f.name,
                                           f.position,
                                           f.module_type,
                                           f.parent_id
                                    FROM file_module f
                                             INNER JOIN file_module_tree fr ON f.parent_id = fr.id)
SELECT DISTINCT *
FROM file_module_tree;

--! get_root_module
SELECT
    id
FROM
    file_module
WHERE
    parent_id IS NULL
    AND project_id = :project_id
    AND module_type = :module_type;


--! insert_file_module (parent_id?)
INSERT INTO
    file_module (project_id, name, position, module_type, parent_id, created_by)
    VALUES (:project_id, :name, :position, :module_type, :parent_id, :created_by)
RETURNING id;

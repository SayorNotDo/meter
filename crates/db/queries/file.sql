--! get_file_modules : (parent_id?)
WITH RECURSIVE file_module_tree AS (SELECT id,
                                           name,
                                           position,
                                           module_type,
                                           parent_id
                                    FROM file_module
                                    WHERE project_id = :project_id
                                    AND module_type = :module_type
                                    AND deleted_at IS NULL
                                    AND deleted_by IS NULL
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

--! get_descendant_by_id : (parent_id?)
SELECT  id,
        name,
        module_type,
        position,
        parent_id
FROM file_module
WHERE parent_id = :parent_id;

--! get_root_module_by_id : (parent_id?)
SELECT  id,
        name,
        module_type,
        position,
        parent_id
FROM file_module
WHERE parent_id IS NULL AND project_id = :project_id AND module_type = :module_type;


--! get_root_module
SELECT
    id
FROM
    file_module
WHERE
    parent_id IS NULL OR parent_id = 0
    AND project_id = :project_id
    AND module_type = :module_type;


--! insert_file_module (parent_id?)
INSERT INTO
    file_module (project_id, name, position, module_type, parent_id, created_by)
    VALUES (:project_id, :name, :position, :module_type, :parent_id, :created_by)
RETURNING id;

--! soft_delete_by_id
UPDATE file_module
SET deleted_at = NOW(),
    deleted_by = :deleted_by,
    updated_by = :deleted_by
WHERE id = :module_id;

--! get_file_module_by_id : (parent_id?)
SELECT
    id,
    name,
    position,
    module_type,
    parent_id
FROM file_module
WHERE id = :id AND deleted_at IS NULL AND deleted_by IS NULL;

--! update_file_module (parent_id?) :
UPDATE  file_module
SET     name = :name,
        parent_id = :parent_id,
        updated_by = :updated_by
WHERE   id = :module_id;

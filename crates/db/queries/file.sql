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

--! insert (description?)
INSERT INTO elements
(name, value, type, description, created_by)
VALUES(:name, :value, :type, :description, :created_by)
RETURNING id;

--! update (description?) :
UPDATE elements
SET
    name = :name,
    value = :value,
    type = :type,
    description = :description,
    updated_by = :updated_by
WHERE
    id = :id;


--! get_element : (value?)
SELECT
    e.id,
    e.name,
    e.type AS element_type,
    e.value,
    oo.name AS option,
    oo.exec AS action
FROM elements e
INNER JOIN operation_option oo ON oo.id = :operation_option_id
WHERE e.id = :id;

--! count
SELECT
    fm.name AS module_name,
    COUNT(e.id) AS element_count
FROM file_module fm
LEFT JOIN elements e
    ON e.module_id  = fm.id
    AND e.deleted = :is_deleted
WHERE
    fm.project_id = :project_id
GROUP BY fm.name;

--! count_by_module_id
SELECT
    COUNT(e.id) AS count
FROM elements e
WHERE
    e.module_id = :module_id
AND deleted = :is_deleted;

--! get_element_list : (updated_at?, updated_by?, description?)
SELECT  e.id,
        e.name,
        (SELECT name FROM file_module WHERE file_module.id = e.module_id) AS module_name,
        e.value,
        e.description,
        e.type AS element_type,
        e.created_at,
        (SELECT name FROM users WHERE users.uuid = e.created_by) AS created_by,
        e.updated_at,
        (SELECT name FROM users WHERE users.uuid = e.updated_by) AS updated_by
FROM elements e
WHERE e.module_id = ANY(SELECT fm.id FROM file_module fm WHERE fm.id = ANY(:module_id) OR fm.parent_id = ANY(:module_id))
AND e.deleted = FALSE
AND e.id >= :start_id
LIMIT :page_size;

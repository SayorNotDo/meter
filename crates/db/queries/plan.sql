--! insert (description?, start_date?, end_date?)
INSERT INTO plans
(name, project_id, description, module_id, created_by, start_date, end_date)
VALUES(:name, :project_id, :description, :module_id, :created_by, :start_date, :end_date)
RETURNING id;


--! count_by_module_id
SELECT
    COUNT(id) AS count
FROM plans
WHERE
    module_id = :module_id
    AND deleted = :is_deleted;

--! count
SELECT
    fm.name AS module_name,
    COUNT(p.id) AS plan_count
FROM file_module fm
LEFT JOIN plans p
    ON p.module_id = fm.id
    AND p.deleted = :is_deleted
WHERE
    p.project_id = :project_id
    AND deleted = :is_deleted
GROUP BY fm.name;


--! get_plan_list :(description?, updated_at?, updated_by?, start_date?, end_date?)
SELECT
    p.id,
    p.name,
    p.status,
    (SELECT name FROM file_module WHERE file_module.id = p.module_id) AS belong_module,
    p.description,
    (SELECT name FROM projects WHERE projects.id = p.project_id) AS belong_project,
    p.created_at,
    (SELECT name FROM users WHERE users.uuid = p.created_by) AS created_by,
    p.updated_at,
    (SELECT name FROM users WHERE users.uuid = p.updated_by) AS updated_by,
    p.start_date,
    p.end_date
FROM plans p
WHERE p.module_id = ANY(SELECT fm.id FROM file_module fm WHERE fm.id = ANY(:module_id) OR fm.parent_id = ANY(:module_id))
AND p.deleted = FALSE
LIMIT :page_size
OFFSET :offset;

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

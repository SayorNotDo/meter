--! insert (description?, start_date?, end_date?)
INSERT INTO plans
(name, project_id, description, module_id, created_by, start_date, end_date)
VALUES(:name, :project_id, :description, :module_id, :created_by, :start_date, :end_date)
RETURNING id;

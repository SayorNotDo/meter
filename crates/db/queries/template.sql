--! get_template_by_project_id : (description?, updated_at?)
SELECT t.id,
       t.name,
       t.project_id,
       t.description,
       t.internal,
       (SELECT name FROM users WHERE users.uuid = t.created_by) AS created_by,
       t.created_at,
       t.updated_at
FROM template t
WHERE t.project_id = :project_id
  AND t.internal = :internal;

--! get_template_custom_field
SELECT tcf.id,
       tcf.template_id,
       tcf.name,
       tcf.field_type,
       tcf.remark,
       tcf.default_value,
       tcf.internal,
       tcf.created_by
FROM template_custom_field tcf
WHERE tcf.template_id = :template_id;

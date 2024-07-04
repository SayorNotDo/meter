--! insert (description?)
INSERT INTO elements
(name, value, type, description, created_by)
VALUES(:name, :value, :type, :description, :created_by)
RETURNING id;

--! update (description?, updated_by?) :
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

--! insert
INSERT INTO elements
(name, value, type, description, created_by)
VALUES(:name, :value, :type, :description, :created_by);

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

--! insert
INSERT INTO elements
(name, value, type, description)
VALUES(:name, :value, :type, :description);

--! get_element : (value?)
SELECT
    e.id,
    e.name,
    e.type AS element_type,
    e.value,
    oo.name AS action,
    oo.exec AS execution
FROM elements e
LEFT JOIN operation_option oo ON oo.id = :operation_option_id
WHERE e.id = :id;

--! insert
INSERT INTO elements
(name, value, type, description)
VALUES(:name, :value, :type, :description);

--! get_element
SELECT
    e.id,
    e.name,
    e.type,
    e.value,
    oo.name AS action,
    oo.exec AS execution
FROM elements e
JOIN operation_option oo ON oo.id = :operation_option_id
WHERE e.id = :id;

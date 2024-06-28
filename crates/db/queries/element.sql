--! insert
INSERT INTO elements
(name, value, type, description)
VALUES(:name, :value, :type, :description);

--! get_elements
SELECT
    e.name,
    e.type,
    e.value
FROM elements e
JOIN script_element_relation ser
ON ser.element_operation_id = :operation_id
AND ser.field_type = :field_type
WHERE e.id = ANY(:ids);

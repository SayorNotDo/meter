--! get_permission_by_role_id
SELECT
    p.id,
    p.module,
    p.scope
FROM role_permission_relation rpr
LEFT JOIN permission p ON p.id = rpr.permission_id
WHERE rpr.role_id = :role_id;


--! get_permission_by_api
SELECT
    p.id,
    p.module,
    p.scope
FROM api_permission_relation apr
LEFT JOIN permission p ON p.id = apr.permission_id
WHERE apr.uri = :uri AND apr.method = :method;


----! get_permission_group_by_role
--SELECT
--    r.id
--FROM role_permission_relation rpr
--LEFT JOIN permission p ON p.id = rpr.permission_id
--WHERE rpr.role_id = ANY();
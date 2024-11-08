--! get_permission_by_role_id
SELECT p.id,
       p.module,
       p.scope
FROM role_permission_relation rpr
         LEFT JOIN permission p ON p.id = rpr.permission_id
WHERE rpr.role_id = :role_id;


--! get_permission_by_api
SELECT p.id,
       p.module,
       p.scope
FROM api_permission_relation apr
         LEFT JOIN permission p ON p.id = apr.permission_id
WHERE apr.uri = :uri
  AND apr.method = :method;


--! get_permission_group_by_role : (updated_at?, description?, updated_by?)
SELECT r.id,
       r.name                                                       AS role_name,
       r.type                                                       AS role_type,
       r.internal,
       (SELECT username FROM users WHERE users.uuid = r.created_by) AS created_by,
       r.created_at,
       r.updated_at,
       (SELECT username FROM users WHERE users.uuid = r.updated_by) AS updated_by,
       r.description,
       COALESCE(JSON_AGG(
                        JSON_BUILD_OBJECT(
                                'id', p.id,
                                'module', p.module,
                                'scope', p.scope
                        )
                ), '[]')                                            AS permission_list
FROM role_permission_relation rpr
         LEFT JOIN user_role r ON r.id = rpr.role_id
         LEFT JOIN permission p ON p.id = rpr.permission_id
GROUP BY r.id;

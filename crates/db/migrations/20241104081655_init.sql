-- migrate:up

-- init internal user
INSERT INTO users
(username,
 uuid,
 hashed_password,
 email)
VALUES ('__system__',
        gen_random_uuid(),
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        'system@test.io');

-- init admin user
INSERT INTO users
(username,
 uuid,
 hashed_password,
 last_project_id,
 email,
 created_by)
VALUES ('admin',
        '24578899-b163-48fe-8594-1fa60134ed2d',
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        1,
        'admin@test.io',
        (SELECT uuid FROM users WHERE username = '__system__'));

-- init system role
INSERT INTO user_role
(id,
 name,
 type,
 internal,
 description,
 created_by)
VALUES (1,
        'SYSTEM',
        'SYSTEM',
        true,
        '系统抽象角色',
        (SELECT uuid FROM users WHERE username = '__system__'));

-- init admin role
INSERT INTO user_role
(id,
 name,
 type,
 internal,
 description,
 created_by)
VALUES (2,
        'ADMIN',
        'PROJECT',
        true,
        '拥有系统全部组织以及项目的操作权限',
        (SELECT uuid FROM users WHERE username = '__system__'));

-- init default project
INSERT INTO projects
(id,
 name,
 created_by,
 enable,
 description,
 module_setting)
VALUES (1,
        '默认项目',
        (SELECT uuid FROM users WHERE username = '__system__'),
        true,
        '系统默认创建的项目',
        '["bugManagement","caseManagement","apiTest","testPlan"]');

-- init system role relation
INSERT INTO user_role_relation
(user_id,
 role_id,
 project_id,
 created_by)
VALUES ((SELECT uuid FROM users WHERE username = 'system'),
        (SELECT id FROM user_role WHERE name = 'SYSTEM'),
        (SELECT id FROM projects WHERE name = '默认项目'),
        (SELECT uuid FROM users WHERE username = '__system__'));

-- init admin role relation
INSERT INTO user_role_relation
(user_id,
 role_id,
 project_id,
 created_by)
VALUES ((SELECT uuid FROM users WHERE username = 'admin'),
        (SELECT id FROM user_role WHERE name = 'ADMIN'),
        (SELECT id FROM projects WHERE name = '默认项目'),
        (SELECT uuid FROM users WHERE username = '__system__'));

-- init permission
INSERT INTO permission
(module,
 scope)
VALUES ('system:role', 'READ'),
       ('system:role', 'WRITE'),
       ('system:user', 'READ'),
       ('system:user', 'WRITE');


-- init api permission
INSERT INTO api_permission_relation
(uri,
 method,
 permission_id)
VALUES ('/system/role/permission/list', 'GET',
        (SELECT id FROM permission WHERE module = 'system:role' AND scope = 'READ')),
       ('/auth/register', 'POST', (SELECT id FROM permission WHERE module = 'system:user' AND scope = 'WRITE'));

-- init internal role permission
INSERT INTO role_permission_relation
    (role_id, permission_id)
VALUES ((SELECT id FROM user_role WHERE name = 'SYSTEM'),
        (SELECT id FROM permission WHERE module = 'system:role' AND scope = 'READ')),
       ((SELECT id FROM user_role WHERE name = 'SYSTEM'),
        (SELECT id FROM permission WHERE module = 'system:role' AND scope = 'WRITE')),
       ((SELECT id FROM user_role WHERE name = 'SYSTEM'),
        (SELECT id FROM permission WHERE module = 'system:user' AND scope = 'READ')),
       ((SELECT id FROM user_role WHERE name = 'SYSTEM'),
        (SELECT id FROM permission WHERE module = 'system:user' AND scope = 'WRITE')),
       ((SELECT id FROM user_role WHERE name = 'ADMIN'),
        (SELECT id FROM permission WHERE module = 'system:role' AND scope = 'READ')),
       ((SELECT id FROM user_role WHERE name = 'ADMIN'),
        (SELECT id FROM permission WHERE module = 'system:role' AND scope = 'WRITE')),
       ((SELECT id FROM user_role WHERE name = 'ADMIN'),
        (SELECT id FROM permission WHERE module = 'system:user' AND scope = 'READ')),
       ((SELECT id FROM user_role WHERE name = 'ADMIN'),
        (SELECT id FROM permission WHERE module = 'system:user' AND scope = 'WRITE'));

-- migrate:down


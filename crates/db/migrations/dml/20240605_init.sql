-- 初始化管理帐户
INSERT INTO users
(username,
 uuid,
 hashed_password,
 email)
VALUES ('admin',
        '24578899-b163-48fe-8594-1fa60134ed2d',
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        'admin@test.io');

-- 初始化组织
INSERT INTO organizations (name, description, created_by)
VALUES ('默认组织',
        '系统默认创建的组织',
        (SELECT uuid FROM users WHERE username = 'admin'));

-- 初始化项目
INSERT INTO projects
(name,
 organization_id,
 created_by,
 description)
VALUES ('默认项目',
        (SELECT id FROM organizations WHERE name = '默认组织'),
        (SELECT uuid FROM users WHERE username = 'admin'),
        '系统默认创建的项目');

-- 初始化用户组
INSERT INTO user_role
(name,
 type,
 description,
 created_by)
VALUES ('admin',
        'SYSTEM',
        '拥有系统全部组织以及项目的操作权限',
        (SELECT uuid FROM users WHERE username = 'admin'));

-- 初始化用户与组的关系
INSERT INTO user_role_relation
(user_id,
 role_id,
 organization_id,
 created_by)
VALUES ((SELECT uuid FROM users WHERE username = 'admin'),
        (SELECT id FROM user_role WHERE name = 'admin'),
        (SELECT id FROM organizations WHERE name = '默认组织'),
        (SELECT uuid FROM users WHERE username = 'admin'));
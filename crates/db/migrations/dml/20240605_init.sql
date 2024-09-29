-- 初始化管理帐户
INSERT INTO users
(username,
 uuid,
 hashed_password,
 last_organization_id,
 last_project_id,
 email)
VALUES ('admin',
        '24578899-b163-48fe-8594-1fa60134ed2d',
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        1,
        1,
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
 enable,
 description,
 module_setting)
VALUES ('默认项目',
        (SELECT id FROM organizations WHERE name = '默认组织'),
        (SELECT uuid FROM users WHERE username = 'admin'),
        true,
        '系统默认创建的项目',
        '["bugManagement","caseManagement","apiTest","testPlan"]');

-- 初始化用户组
INSERT INTO user_role
(name,
 type,
 internal,
 description,
 created_by)
VALUES ('admin',
        'SYSTEM',
        true,
        '拥有系统全部组织以及项目的操作权限',
        (SELECT uuid FROM users WHERE username = 'admin'));

-- 初始化用户与组的关系
INSERT INTO user_role_relation
(user_id,
 role_id,
 project_id,
 created_by)
VALUES ((SELECT uuid FROM users WHERE username = 'admin'),
        (SELECT id FROM user_role WHERE name = 'admin'),
        (SELECT id FROM projects WHERE name = '默认项目'),
        (SELECT uuid FROM users WHERE username = 'admin'));

-- 初始化功能用例模版
INSERT INTO template
(name,
 project_id,
 description,
 internal,
 created_by)
VALUES ('默认模板',
        1,
        '系统默认创建的功能用例模版',
        true,
        (SELECT uuid FROM users WHERE username = 'admin'));

-- 文件树调试数据导入
INSERT INTO file_module
    (project_id, name, module_type, created_by)
VALUES (1, 'root', 'ELEMENT', '24578899-b163-48fe-8594-1fa60134ed2d');

INSERT INTO file_module
    (project_id, name, module_type, created_by)
VALUES (1, 'another', 'ELEMENT', '24578899-b163-48fe-8594-1fa60134ed2d');

INSERT INTO file_module
    (project_id, name, module_type, parent_id, created_by)
VALUES (1, 'sub', 'ELEMENT', 1, '24578899-b163-48fe-8594-1fa60134ed2d');

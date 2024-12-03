-- migrate:up
-- init internal user
INSERT INTO
    users (
        username,
        uuid,
        hashed_password,
        email,
        created_by
    )
VALUES
    (
        '__system__',
        '24578899-b163-48fe-8594-1fa60134ed2d',
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        'system@test.io',
        '24578899-b163-48fe-8594-1fa60134ed2d'
    );

-- init admin user
INSERT INTO
    users (
        username,
        uuid,
        hashed_password,
        last_project_id,
        email,
        created_by
    )
VALUES
    (
        'admin',
        gen_random_uuid (),
        '$argon2id$v=19$m=19456,t=2,p=1$NskOoxLFUtTPzhT4UyNCSw$u1FSg95/l5fQ5EzyQWod7aknDyitqhAcUjePnLH/pBg',
        1,
        'admin@test.io',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- init system role
INSERT INTO
    user_role (name, type, internal, description, created_by)
VALUES
    (
        'SYSTEM',
        'SYSTEM',
        true,
        '系统抽象角色',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- init admin role
INSERT INTO
    user_role (name, type, internal, description, created_by)
VALUES
    (
        'ADMIN',
        'PROJECT',
        true,
        '拥有系统全部组织以及项目的操作权限',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- init default project
INSERT INTO
    projects (
        name,
        created_by,
        enable,
        description,
        module_setting
    )
VALUES
    (
        '默认项目',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        ),
        true,
        '系统默认创建的项目',
        '["bugManagement","caseManagement","apiTest","testPlan"]'
    );

-- init system role relation
INSERT INTO
    user_role_relation (user_id, role_id, project_id, created_by)
VALUES
    (
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        ),
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'SYSTEM'
        ),
        (
            SELECT
                id
            FROM
                projects
            WHERE
                name = '默认项目'
        ),
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- init admin role relation
INSERT INTO
    user_role_relation (user_id, role_id, project_id, created_by)
VALUES
    (
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = 'admin'
        ),
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                projects
            WHERE
                name = '默认项目'
        ),
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- init permission
INSERT INTO
    permission (module, scope)
VALUES
    ('SYSTEM:ROLE', 'READ'),
    ('SYSTEM:ROLE', 'WRITE'),
    ('SYSTEM:USER', 'READ'),
    ('SYSTEM:USER', 'WRITE'),
    ('MANAGEMENT:FUNCTIONAL_CASE', 'READ'),
    ('MANAGEMENT:FUNCTIONAL_CASE', 'WRITE');

-- init api permission
INSERT INTO
    api_permission_relation (uri, method, permission_id)
VALUES
    (
        '/system/role/permission/list',
        'GET',
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:ROLE'
                AND scope = 'READ'
        )
    ),
    (
        '/auth/register',
        'POST',
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'WRITE'
        )
    ),
    (
        '/system/user/list',
        'GET',
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'READ'
        )
    );

-- init internal role permission
INSERT INTO
    role_permission_relation (role_id, permission_id)
VALUES
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'SYSTEM'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:ROLE'
                AND scope = 'READ'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'SYSTEM'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:ROLE'
                AND scope = 'WRITE'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'SYSTEM'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'READ'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'SYSTEM'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'WRITE'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:ROLE'
                AND scope = 'READ'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:ROLE'
                AND scope = 'WRITE'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'READ'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'SYSTEM:USER'
                AND scope = 'WRITE'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'MANAGEMENT:FUNCTIONAL_CASE'
                AND scope = 'READ'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                user_role
            WHERE
                name = 'ADMIN'
        ),
        (
            SELECT
                id
            FROM
                permission
            WHERE
                module = 'MANAGEMENT:FUNCTIONAL_CASE'
                AND scope = 'WRITE'
        )
    );

-- init internal template
INSERT INTO
    template (
        name,
        project_id,
        description,
        internal,
        created_by
    )
VALUES
    (
        '功能测试用例模版',
        (
            SELECT
                id
            FROM
                projects
            WHERE
                name = '默认项目'
        ),
        '功能测试用例模版，用于规范化测试用例',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

INSERT INTO
    field (
        name,
        label,
        field_type,
        project_id,
        remark,
        internal,
        created_by
    )
VALUES
    (
        'caseNum',
        '用例编号',
        'INPUT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        'priority',
        '等级',
        'SELECT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        'preRequisite',
        '前置条件',
        'INPUT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        'step',
        '测试步骤',
        'INPUT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        'expectedResult',
        '预期结果',
        'INPUT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        'remark',
        '备注',
        'INPUT',
        1,
        '',
        true,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

INSERT INTO
    field_option (field_id, value, position, created_by)
VALUES
    (
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'priority'
        ),
        'P0',
        1,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'priority'
        ),
        'P1',
        2,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'priority'
        ),
        'P2',
        3,
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

INSERT INTO
    template_field_relation (
        template_id,
        field_id,
        required,
        default_value,
        created_by
    )
VALUES
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'caseNum'
        ),
        true,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'priority'
        ),
        true,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'preRequisite'
        ),
        true,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'step'
        ),
        true,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'expectedResult'
        ),
        false,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    ),
    (
        (
            SELECT
                id
            FROM
                template
            WHERE
                name = '功能测试用例模版'
        ),
        (
            SELECT
                id
            FROM
                field
            WHERE
                name = 'remark'
        ),
        false,
        '',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

INSERT INTO
    file_module (project_id, name, module_type, created_by)
VALUES
    (
        1,
        '示例文件夹',
        'CASE',
        (
            SELECT
                uuid
            FROM
                users
            WHERE
                username = '__system__'
        )
    );

-- migrate:down

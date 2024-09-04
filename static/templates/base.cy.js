{% macro process_steps(steps) %}
{% for raw in steps %}
{% if raw.action == 'CLICK' %}
cy.get('{{ raw.selector }}').click()
{% elif raw.action == 'TYPE' %}
    cy.get('{{ raw.selector }}').type('{{ raw.value }}')
{% elif raw.action == 'VISIT' %}
    {% for key, value in raw.value %}
    {% if key == 'default' %}
    cy.visit('{{ value }}')
    {% endif %}
    {% endfor %}
{% elif raw.action == 'CLEAR' %}
    cy.get('{{ raw.selector }}').clear()
{% elif raw.action == 'CHECK' %}
    cy.get('{{ raw.selector }}').check()
{% elif raw.action == 'UNCHECK' %}
    cy.get('{{ raw.selector }}').uncheck()
{% elif raw.action == 'DBLCLICK' %}
    cy.get('{{ raw.selector }}').dblclick()
{% elif raw.action == 'RCLICK' %}
    cy.get('{{ raw.selector }}').rightclick()
{% elif raw.action == 'SELECT' %}
    cy.get('{{ raw.selector }}').select('{{ raw.value }}')
{% elif raw.action == 'REQUEST' %}
    cy.request({
        {% for key, value in raw.value %}
        {% if key == 'url' %}
            url: '{{ value }}',
        {% endif %}
        {% if key == 'method' %}
            method: '{{ value }}',
        {% endif %}
        {% if key == 'headers' %}
            headers: {{ value }},
        {% endif %}
        {% if key == 'body' %}
            body: {{ value }},
        {% endif %}
        {% endfor %} })
    {% endif %}
    {% if raw.expected %}
    {% for key, value in raw.expected %}
        {% if key == "type" %}.should('{{ value }}'{% endif %}{% if key == "value" %},'{{ value }}'){% else %}){% endif %}
    {% endfor %}
{% endif %}
{% endfor %}
{% endmacro process_steps %}

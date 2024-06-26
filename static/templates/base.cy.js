{% macro process_steps(steps) %}
{% for raw in steps %}
{% if raw.action == 'CLICK' %}
cy.get('{{ raw.selector }}').click()
{% elif raw.action == 'TYPE' %}
cy.get('{{ raw.selector }}').type('{{ raw.input }}')
{% endif %}
{% endfor %}
{% endmacro process_steps %}

{% macro process_steps(steps) %}
    {% for raw in steps %}
        {% if raw.action == 'CLICK' %}
            cy.get('{{ raw.selector }}').click()
        {% elif raw.action == 'TYPE' %}
            cy.get('{{ raw.selector }}').type('{{ raw.value }}')
        {% elif raw.action == 'VISIT' %}
            cy.visit('{{ raw.value }}')
        {% elif raw.action == 'CLEAR' %}
            cy.get('{{ raw.selector }}').clear()
        {% endif %}
        {% if raw.expected is defined %}
            {% for key, value in raw.expected.items() %}
                .should('{{ key }}', '{{ value }}')
            {% endfor %}
        {% endif %}
    {% endfor %}
{% endmacro process_steps %}

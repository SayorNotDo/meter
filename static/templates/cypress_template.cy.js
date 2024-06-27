{% import "base.cy.js" as macros %}
describe('{{ name }}', () => {
    {% if pre_processors is defined and pre_processors | length > 0 %}
    beforeEach(() => {
    {{ macros::process_steps(steps=pre_processors) | remove_empty_lines }}
    })
    {% endif %}{% if after_processors is defined and after_processors | length > 0 %}
    afterEach(() => {
    {{ macros::process_steps(steps=after_processors) | remove_empty_lines }}
    })
    {% endif %}
    it('{{ description }}', () => {
    {% if case_steps is defined and case_steps | length > 0 %}{{ macros::process_steps(steps=case_steps) | remove_empty_lines }}
    {% endif %}});
})

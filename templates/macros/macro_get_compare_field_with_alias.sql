{% macro macro_get_compare_field_with_alias(alias, field) %}
    {% if ' AS ' in field %}
        {% set list1 = field | split(pat=" AS ") %}
        {{ list1[1] }}
    {% else %}
        {{ alias ~ '.' ~field }}
    {% endif %}
{% endmacro %}
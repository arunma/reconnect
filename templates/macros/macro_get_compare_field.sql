{% macro macro_get_compare_field(fields) %}
    {% for field in fields %}
        {% set list1 = field | split(pat=" AS ") %}
        {% if loop.first -%}
            {{ list1[0] }}
        {% else %}
        , {{ list1[0] }}
        {% endif %}
    {% endfor %}

{% endmacro %}
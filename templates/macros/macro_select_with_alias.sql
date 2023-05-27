{% macro macro_select_with_alias(fields, alias) %}
  {% for field in fields %}
    {% set statement = ''%}
    {% if ' AS ' in field %}
        {% set statement = field  %}
    {% else %}
        {% set statement = alias ~ '.' ~field ~ ' AS ' ~ alias ~ "__" ~ field%}
    {% endif %}
    {% if loop.first -%}
        {{ statement }}
    {% else %}
    , {{ statement }}
    {% endif %}
  {% endfor %}
{% endmacro %}
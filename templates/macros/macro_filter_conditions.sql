{% macro macro_filter_conditions(filters) %}
  {% for filter in filters %}
    {% if loop.first %}
      {{ filter }}
    {% else %}
      AND {{ filter }}
    {% endif %}
  {% endfor %}
{% endmacro %}
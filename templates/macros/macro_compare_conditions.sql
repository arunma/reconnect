{% import 'macros/macro_get_compare_field_with_alias.sql' as macro_get_compare_field_with_alias %}
{% macro macro_compare_conditions(left_alias, right_alias, left_fields, right_fields) %}
    {% for left_field in left_fields %}
        {% set right_field = right_fields | nth(n=loop.index-1) %}
        {% if loop.first %}
          {{ macro_get_compare_field_with_alias::macro_get_compare_field_with_alias(alias=left_alias,field=left_field) }} != {{ macro_get_compare_field_with_alias::macro_get_compare_field_with_alias(alias=right_alias, field=right_field ) }}
        {% else %}
    OR
    {{ macro_get_compare_field_with_alias::macro_get_compare_field_with_alias(alias=left_alias,field=left_field) }} != {{ macro_get_compare_field_with_alias::macro_get_compare_field_with_alias(alias=right_alias, field=right_field ) }}
        {%- endif %}
    {% endfor %}
{% endmacro %}
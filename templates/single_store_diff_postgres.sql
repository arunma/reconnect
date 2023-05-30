{% import 'macros/macro_compare_conditions.sql' as macro_compare_conditions %}
{% import 'macros/macro_filter_conditions.sql' as macro_filter_conditions %}
{% import 'macros/macro_select_with_alias.sql' as macro_select_with_alias %}

SELECT
        {{ macro_select_with_alias::macro_select_with_alias(fields=left_key, alias=left_alias) }}
    ,    {{ macro_select_with_alias::macro_select_with_alias(fields=right_key, alias=right_alias) }}
    ,   CASE
            {% for lkey in left_key %}
                WHEN ({{ left_alias }}.{{ lkey}}) is null THEN 'RO'
            {% endfor %}

            {% for rkey in right_key %}
                WHEN ({{ right_alias }}.{{ rkey}}) is null THEN 'LO'
            {% endfor %}
            ELSE 'DF'
        END AS status
        {% if left_satellite_fields %}
    ,   {{ macro_select_with_alias::macro_select_with_alias (fields=left_satellite_fields, alias=left_alias) }}
        {% endif %}
        {% if right_satellite_fields %}
    ,   {{ macro_select_with_alias::macro_select_with_alias (fields=right_satellite_fields, alias=right_alias) }}
        {% endif %}
    ,   {{ macro_select_with_alias::macro_select_with_alias (fields=left_compare_fields, alias=left_alias) }}
    ,   {{ macro_select_with_alias::macro_select_with_alias (fields=right_compare_fields, alias=right_alias) }}
FROM
    {{ left_table }} AS {{ left_alias }}
FULL OUTER JOIN
    {{ right_table }} AS {{ right_alias }}
ON
    {% for i in left_key %}
        {% if not loop.first -%}
            AND
        {% endif %}
        {{ left_alias }}.{{ left_key[loop.index0] }} = {{ right_alias }}.{{right_key[loop.index0]}}
    {% endfor %}
WHERE
    (
    {% for lkey in left_key%}
        {% if not loop.first -%}
            OR
        {% endif %}
        {{ left_alias }}.{{ lkey }} is null
    {% endfor %}

    {% for rkey in right_key%}
        OR {{ right_alias }}.{{ rkey }} is null
    {% endfor %}
    OR
    {{ macro_compare_conditions::macro_compare_conditions(left_alias=left_alias, right_alias=right_alias, left_fields=left_compare_fields, right_fields=right_compare_fields) }}
    )
    {% if left_filter_conditions %}
AND
    {{ macro_filter_conditions::macro_filter_conditions(filters=left_filter_conditions) }}
    {% endif %}
    {% if right_filter_conditions %}
AND
    {{ macro_filter_conditions::macro_filter_conditions(filters=right_filter_conditions) }}
    {% endif %}

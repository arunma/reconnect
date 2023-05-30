{% import 'macros/macro_filter_conditions.sql' as macro_filter_conditions %}
{% import 'macros/macro_select_with_alias_postgres.sql' as macro_select_with_alias_postgres %}

SELECT
    {{ macro_select_with_alias_postgres (fields=params.key, alias=params.alias) }}
    , CONCAT({{params.key | join (sep=",")}}) AS "KEY"
	{% if params.satellite_fields %}
	, {{ macro_select_with_alias_postgres (fields=params.satellite_fields, alias=params.alias) }}
	{% endif %}
    , {{ macro_select_with_alias_postgres (fields=params.compare_fields, alias=params.alias) }}
FROM
    {{ params.table }} AS {{ params.alias }}
WHERE
    CONCAT({{params.key | join (sep=",")}}) in ('{{ params.diff_ids| join(sep="','") }}')
{% if params.filter_conditions %}
AND
    {{ macro_filter_conditions(filters=params.filter_conditions) }}
{% endif %}

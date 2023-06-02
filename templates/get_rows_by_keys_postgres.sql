{% import 'macros/macro_filter_conditions.sql' as macro_filter_conditions %}
{% import 'macros/macro_select_with_alias_postgres.sql' as macro_select_with_alias_postgres %}

SELECT
    {{ macro_select_with_alias_postgres::macro_select_with_alias_postgres (fields=key, alias=alias) }}
    , CONCAT({{key | join (sep=",")}}) AS "KEY"
	{% if satellite_fields %}
	, {{ macro_select_with_alias_postgres::macro_select_with_alias_postgres (fields=satellite_fields, alias=alias) }}
	{% endif %}
    , {{ macro_select_with_alias_postgres::macro_select_with_alias_postgres (fields=compare_fields, alias=alias) }}
FROM
    {{ table }} AS {{ alias }}
WHERE
    CONCAT({{key | join (sep=",")}}) in ('{{ diff_keys| join(sep="','") }}')
{% if filter_conditions %}
AND
    {{ macro_filter_conditions::macro_filter_conditions(filters=filter_conditions) }}
{% endif %}

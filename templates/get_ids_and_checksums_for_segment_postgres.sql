{% import 'macros/macro_filter_conditions.sql' as macro_filter_conditions %}

SELECT
      CONCAT_WS ('___', CONCAT({{ key | join(sep=",") }})
    , ('x' || SUBSTRING(MD5 (CONCAT({{compare_fields | join (sep=",")}})), 18))::bit(60)::bigint) AS id_checksum
FROM
    {{ table }} AS {{ alias }}
WHERE 1=1
{% if filter_conditions %}
AND
    {{ macro_filter_conditions::macro_filter_conditions(filters=filter_conditions) }}
{% endif %}
AND
    CONCAT({{ key | join(sep=",") }}) BETWEEN '{{ seg_min }}' AND '{{ seg_max }}'

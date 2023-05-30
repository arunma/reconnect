{% import 'macros/macro_filter_conditions.sql' as macro_filter_conditions %}

WITH md5s AS
(
	SELECT
	      CONCAT( {{ key | join (sep=",") }}) AS key
        , ('x' || SUBSTRING(MD5 (CONCAT( {{compare_fields | join (sep=",")}} )), 18))::bit(60)::bigint AS csums
	FROM
	    {{ table }} AS {{ alias }}
	{% if filter_conditions %}
	WHERE
	    {{ macro_filter_conditions::macro_filter_conditions(filters=filter_conditions) }}
	{% endif %}
	ORDER BY csums
), grouped_csums AS
(
	SELECT
	        MIN(key) as seg_min
	    ,   MAX(key) as seg_max
	    ,   COUNT(*) as seg_count
	    ,   SUM(csums) as seg_checksum
	FROM md5s
)
SELECT
    *
FROM
    grouped_csums;

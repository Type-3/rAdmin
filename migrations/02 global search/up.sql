-- Your SQL goes here
CREATE OR REPLACE FUNCTION radmin_global_search(
    needle text,
    haystack_tables name[] default '{}',
    haystack_schema name default 'public'
)
RETURNS table(schemaname text, tablename text, columnname text, rowid uuid, field_value text)
AS $$

begin
  FOR schemaname,tablename,columnname IN
      SELECT c.table_schema,c.table_name,c.column_name
      FROM information_schema.columns c
        JOIN information_schema.tables t ON
          (t.table_name=c.table_name AND t.table_schema=c.table_schema)
        JOIN information_schema.table_privileges p ON
          (t.table_name=p.table_name AND t.table_schema=p.table_schema
              AND p.privilege_type='SELECT')
        JOIN information_schema.schemata s ON
          (s.schema_name=t.table_schema)
      WHERE (c.table_name=ANY(haystack_tables) OR haystack_tables='{}')
        AND (c.table_schema=haystack_schema OR haystack_schema='public')
        AND t.table_type='BASE TABLE'
  LOOP
    FOR rowid, field_value IN
      EXECUTE format('SELECT id,%I FROM %I.%I WHERE LOWER(cast(%I as text)) LIKE %L',
	   columnname,
       schemaname,
       tablename,
       columnname,
       '%' || LOWER(needle) || '%'
      )
    LOOP
	  RAISE NOTICE 'rowcid: %, field_value: %', rowid, field_value;
      IF rowid is not null THEN
          RETURN NEXT;
      END IF;
      RETURN NEXT;
    END LOOP;
 END LOOP;
END;
$$ language plpgsql;

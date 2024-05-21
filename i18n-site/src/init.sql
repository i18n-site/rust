CREATE TABLE IF NOT EXISTS ver_prefix (
  id bigserial NOT NULL, site_id bigint NOT NULL, prefix character varying(255) NOT NULL, PRIMARY KEY (id), UNIQUE (site_id, prefix)
);

CREATE TABLE IF NOT EXISTS ver (
  id bigserial NOT NULL, ver_prefix_id bigint NOT NULL, name character varying(255) NOT NULL, upload_id bigint NOT NULL, ts bigint NOT NULL DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) ::bigint, uts bigint NOT NULL DEFAULT 0, PRIMARY KEY (id), UNIQUE (ver_prefix_id, name)
);

DROP FUNCTION IF EXISTS ver_li;

CREATE FUNCTION ver_li (_site_id bigint, prefix_li text[], ver_li text[], upload_id_li bigint[])
  RETURNS text
  AS $$
DECLARE
  t json;
  result text := '';
  _prefix text;
  v_prefix_id bigint;
  i int := 1;
BEGIN
  FOREACH _prefix IN ARRAY prefix_li LOOP
    SELECT id INTO v_prefix_id
    FROM ver_prefix
    WHERE site_id = _site_id
      AND prefix = _prefix;
    IF v_prefix_id IS NULL THEN
      INSERT INTO ver_prefix (site_id, prefix)
        VALUES (_site_id, _prefix)
      RETURNING id INTO v_prefix_id;
    END IF;
    INSERT INTO ver (ver_prefix_id, name, upload_id)
      VALUES (v_prefix_id, ver_li[i], upload_id_li[i])
    ON CONFLICT (ver_prefix_id, name)
      DO NOTHING;
    SELECT JSON_AGG(JSON_BUILD_ARRAY(name, upload_id)) INTO t
    FROM ver
    WHERE ver_prefix_id = v_prefix_id;
    IF LENGTH(result) > 0 THEN
      result := result || ',';
    END IF;
    result := result || t::text;
    i := i + 1;
  END LOOP;
  RETURN result;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS ver_log (
  id bigserial NOT NULL, ver_id bigint NOT NULL, upload_id bigint NOT NULL, ts bigint NOT NULL, PRIMARY KEY (id), UNIQUE (ver_id, upload_id)
);

CREATE OR REPLACE FUNCTION log_and_update_ver ()
  RETURNS TRIGGER
  AS $$
BEGIN
  INSERT INTO ver_log (ver_id, upload_id, ts)
    VALUES (OLD.id, OLD.upload_id, OLD.ts)
  ON CONFLICT (ver_id, upload_id)
    DO NOTHING;
  NEW.uts := EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::bigint;
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS tr_ver_before_update ON ver;

CREATE TRIGGER tr_ver_before_update
  BEFORE UPDATE ON ver
  FOR EACH ROW
  WHEN (OLD.upload_id IS DISTINCT FROM NEW.upload_id)
  EXECUTE FUNCTION log_and_update_ver ();

CREATE TABLE IF NOT EXISTS upload (
  id bigint NOT NULL, len bigint NOT NULL, hash bytea NOT NULL, site_id bigint NOT NULL, ts bigint NOT NULL DEFAULT 0, PRIMARY KEY (id), UNIQUE (len, hash)
);

CREATE OR REPLACE FUNCTION uploaded (id_li bigint[])
  RETURNS VOID
AS $$
  UPDATE upload SET ts=EXTRACT(EPOCH FROM CURRENT_TIMESTAMP)::bigint WHERE id = ANY(id_li);
$$
LANGUAGE SQL;

CREATE OR REPLACE FUNCTION upload_id_ts (_site_id bigint, _len_li bigint[], _hash_li bytea[])
  RETURNS TABLE (
    id bigint,
    ts bigint
  )
  AS $$
DECLARE
  _max_id bigint;
  _id bigint;
  _ts bigint;
  _len bigint;
  _hash bytea;
BEGIN
  _max_id: = 0;
  FOR I IN 1..ARRAY_LENGTH(_len_li, 1)
  LOOP
    _len := _len_li[I];
    _hash := _hash_li[I];
    SELECT t.id, t.ts INTO _id, _ts
    FROM upload t
    WHERE len = _len
      AND hash = _hash AND site_id=_site_id;
    IF _id IS NULL THEN
      IF _max_id = 0 THEN
        SELECT nextId('upload') INTO _max_id;
      ELSE
        _max_id: = _max_id + 1;
      END IF;
      LOOP
        INSERT INTO upload (id, len, hash, site_id)
          VALUES (_max_id, _len, _hash, _site_id)
        ON CONFLICT (len, hash)
          DO NOTHING
        RETURNING upload.id INTO _id;
        IF _id = _max_id THEN
          EXIT;
        ELSE
          _max_id := _max_id + 1;
        END IF;
      END LOOP;
      _ts := 0;
    END IF;
    id := _id;
    ts := _ts;
    RETURN NEXT;
  END LOOP;
END;
$$
LANGUAGE 'plpgsql';

CREATE OR REPLACE FUNCTION nextId (table_name text)
  RETURNS integer
  AS $$
DECLARE
  max_id integer;
BEGIN
  EXECUTE FORMAT('SELECT COALESCE(MAX(id), 0) FROM %I', table_name) INTO max_id;
  RETURN max_id + 1;
END;
$$
LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS site (
  id bigserial NOT NULL, name character varying(255), upload_id bigint NOT NULL DEFAULT 0, ts bigint NOT NULL DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) ::bigint, PRIMARY KEY (id), UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS site_upload_id_log (
  id bigserial NOT NULL, site_id bigint NOT NULL, upload_id bigint NOT NULL, ts bigint NOT NULL DEFAULT EXTRACT(EPOCH FROM CURRENT_TIMESTAMP) ::bigint, PRIMARY KEY (id), UNIQUE (site_id, upload_id)
);

CREATE OR REPLACE FUNCTION log_site_upload_change ()
  RETURNS TRIGGER
  AS $$
DECLARE
  _ts bigint;
BEGIN
  SELECT EXTRACT(EPOCH FROM NOW()) INTO _ts;
  NEW.ts = _ts;
  IF OLD.upload_id > 0 THEN
    INSERT INTO site_upload_id_log (site_id, upload_id, ts)
      VALUES (OLD.id, OLD.upload_id, OLD.ts)
    ON CONFLICT (site_id, upload_id)
      DO UPDATE SET
        ts = _ts;
  END IF;
  RETURN NEW;
END;
$$
LANGUAGE 'plpgsql';

DROP TRIGGER IF EXISTS tr_site_upload_change ON site;

CREATE TRIGGER tr_site_upload_change
  BEFORE UPDATE ON site
  FOR EACH ROW
  WHEN (OLD.upload_id IS DISTINCT FROM NEW.upload_id)
  EXECUTE FUNCTION log_site_upload_change ();

CREATE OR REPLACE FUNCTION site_id_upload_id (p_name character varying)
  RETURNS TABLE (
    id bigint,
    upload_id bigint
  )
  AS $$
  WITH i AS (
    SELECT id, upload_id
    FROM site
    WHERE name = p_name
), j AS (
INSERT INTO site (id, name)
  SELECT (
      SELECT nextId ('site')), p_name
  WHERE NOT EXISTS (
      SELECT 1
      FROM i)
    RETURNING id, 0 AS upload_id
)
SELECT id, upload_id
FROM i
UNION ALL
SELECT id, upload_id
FROM j
$$
LANGUAGE SQL;


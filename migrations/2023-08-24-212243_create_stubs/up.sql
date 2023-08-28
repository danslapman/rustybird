CREATE TYPE scope AS ENUM ('persistent', 'ephemeral', 'countdown');

CREATE TYPE http_method AS ENUM ('get', 'post', 'head', 'options', 'patch', 'put', 'delete');

CREATE TABLE stub (
  id SERIAL PRIMARY KEY,
  created TIMESTAMPTZ NOT NULL,
  scope scope NOT NULL,
  times BIGINT NULL,
  service_suffix VARCHAR(40) NOT NULL,
  name VARCHAR(40) NOT NULL,
  method http_method NOT NULL,
  path VARCHAR(256) NULL,
  path_pattern VARCHAR(256) NULL,
  seed JSONB NULL,
  state JSONB NULL,
  request JSONB NOT NULL,
  persist JSONB NULL,
  response JSONB NOT NULL
)
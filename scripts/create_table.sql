-- start with: psql -h 127.0.0.1 -p 5432 -U postgres -v password=foo -f create_table.sql
-- initialize environment
CREATE USER forensics WITH PASSWORD :'password';
CREATE DATABASE forensics OWNER forensics;
GRANT ALL PRIVILEGES ON DATABASE forensics TO forensics;

-- create the FILES table
CREATE TABLE IF NOT EXISTS artefact (
    id integer,
    path text,
    name text,
    ext text,
    type text,
    len  bigint,
    created timestamp,
    accessed timestamp,
    modified timestamp,
    sha256 text,
    blake3  text,
    entropy float
);
ALTER TABLE artefact OWNER TO forensics;
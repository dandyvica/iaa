-- start with: psql -h 127.0.0.1 -p 5432 -U postgres -v password=foo -f create_all.sql
-- initialize environment
-- connect to DB: \c forensics
CREATE USER forensics WITH PASSWORD :'password';
CREATE DATABASE forensics OWNER forensics;
GRANT ALL PRIVILEGES ON DATABASE forensics TO forensics;

-- artefacts table
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
    entropy float,
    mime text,
    metadata jsonb
);

-- set comments on table
COMMENT ON COLUMN artefact.path is 'The full file or directory path';
COMMENT ON COLUMN artefact.name is 'The file or directory name';
COMMENT ON COLUMN artefact.ext is 'The file extension';
COMMENT ON COLUMN artefact.type is 'The artefact type: "F" for file, "D" for directory, "S" for a symbolic link, "U" for unknown';
COMMENT ON COLUMN artefact.len is 'The file size in bytes';

-- store the run history
CREATE TABLE IF NOT EXISTS run_history (
    start_time timestamp,
    end_time timestamp,
    elapsed text,
    nb_files bigint,
    args text,
    tags text
);


ALTER TABLE artefact OWNER TO forensics;
ALTER TABLE run_history OWNER TO forensics;
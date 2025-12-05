-- create the FILES table
CREATE TABLE IF NOT EXISTS files (
    id integer,
    path text,
    name text,
    ext text,
    type text,
    len  integer,
    created timestamp,
    accessed timestamp,
    modified timestamp,
    sha256 text,
    blake3  text,
    entropy float
);
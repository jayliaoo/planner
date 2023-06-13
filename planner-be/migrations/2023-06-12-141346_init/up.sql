-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "user"
(
    "id"   SERIAL PRIMARY KEY NOT NULL,
    "name" VARCHAR(50)        NOT NULL,
    "role" VARCHAR(4)         NOT NULL
);

CREATE TABLE IF NOT EXISTS sprint
(
    "id"    SERIAL PRIMARY KEY NOT NULL,
    "name"  VARCHAR(50)        NOT NULL,
    "start" DATE               NOT NULL,
    "end"   DATE               NOT NULL
);

CREATE SEQUENCE task_id START 1;
CREATE TABLE IF NOT EXISTS task
(
    "id"         SERIAL PRIMARY KEY NOT NULL,
    "name"       VARCHAR(50)        NOT NULL,
    "sprint"     INTEGER            NOT NULL,
    "ordinal"    SMALLINT           NOT NULL,
    "developer"  INTEGER            NOT NULL,
    "sp"         DECIMAL(2, 1)      NOT NULL,
    "tester"     INTEGER,
    "test_sp"    DECIMAL(2, 1),
    "start"      TIMESTAMP,
    "end"        TIMESTAMP,
    "test_start" TIMESTAMP,
    "test_end"   TIMESTAMP
);

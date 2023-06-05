-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `user`
(
    `id`   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `name` VARCHAR(50)                       NOT NULL
);

CREATE TABLE IF NOT EXISTS `sprint`
(
    `id`    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `name`  VARCHAR(50)                       NOT NULL,
    `start` DATE                              NOT NULL,
    `end`   DATE                              NOT NULL
);

CREATE TABLE IF NOT EXISTS `task`
(
    `id`         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    `name`       VARCHAR(50)                       NOT NULL,
    `sprint`     INTEGER                           NOT NULL,
    `ordinal`    SMALLINT                          NOT NULL,
    `developer`  INTEGER                           NOT NULL,
    `sp`         DECIMAL(2, 1)                     NOT NULL,
    `tester`     INTEGER,
    `test_sp`    DECIMAL(2, 1),
    `start`      DATETIME,
    `end`        DATETIME,
    `test_start` DATETIME,
    `test_end`   DATETIME
);

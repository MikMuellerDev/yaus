-- Creates the URL table
CREATE TABLE
IF NOT EXISTS
url(
    short       VARCHAR(20)     NOT NULL,
    target_url      VARCHAR(500)    NOT NULL,
    PRIMARY KEY(short)
)

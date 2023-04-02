CREATE TABLE teams (
  slug        VARCHAR NOT NULL PRIMARY KEY,
  title       VARCHAR NOT NULL
);

INSERT INTO teams (slug, title, is_private, is_accepted) VALUES ('', 'Global', false, true);
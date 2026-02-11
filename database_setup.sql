CREATE TABLE users (
  user_id SERIAL UNIQUE PRIMARY KEY NOT NULL,
  username varchar(255) UNIQUE NOT NULL,
  password varchar(255) NOT NULL
);

CREATE TABLE refresh_tokens (
  user_id integer UNIQUE PRIMARY KEY NOT NULL,
  refresh_token varchar(255) NOT NULL
);

ALTER TABLE refresh_tokens ADD FOREIGN KEY (user_id) REFERENCES users (user_id);
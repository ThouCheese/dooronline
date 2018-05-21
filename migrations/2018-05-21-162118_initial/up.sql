-- Your SQL goes here

CREATE TABLE public."user"
(
    id SERIAL PRIMARY KEY,
    username text NOT NULL,
    password text NOT NULL,
    is_admin boolean NOT NULL
);
CREATE UNIQUE INDEX user_name_uindex ON public."user" (username);
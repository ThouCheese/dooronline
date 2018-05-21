-- Your SQL goes here

CREATE TABLE public."user"
(
    id integer DEFAULT nextval('user_id_seq'::regclass) PRIMARY KEY NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    is_admin boolean NOT NULL
);
CREATE UNIQUE INDEX user_pkey ON public."user" (id);
CREATE UNIQUE INDEX user_name_uindex ON public."user" (username);
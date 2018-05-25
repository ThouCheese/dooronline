-- Your SQL goes here

CREATE TABLE public.log
(
	id SERIAL PRIMARY KEY,
	user_id INT NOT NULL,
	date TIMESTAMP NOT NULL,
    CONSTRAINT log_to_user FOREIGN KEY (user_id) REFERENCES "user" (id)
);

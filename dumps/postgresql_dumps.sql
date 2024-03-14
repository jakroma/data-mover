CREATE TABLE public.tbl_users (
	id UUID PRIMARY KEY,
	type SMALLINT DEFAULT 0 NOT NULL REFERENCES public.dic_user_types(id) ON DELETE CASCADE,
	gender CHAR,
	avatar_id SMALLINT,
	name TEXT NOT NULL,
	description VARCHAR(300),
	created TIMESTAMP WITH TIME ZONE NOT NULL,
	updated TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE public.dic_user_types (
	id SMALLINT PRIMARY KEY,
	value TEXT
);

INSERT INTO public.dic_user_types (id, value)
VALUES (0, 'Normal'), (1, 'Admin');

COMMENT ON TABLE tbl_users IS '';
COMMENT ON COLUMN tbl_users.name IS '';


INSERT INTO public.tbl_users (id, name, type, gender, avatar_id, created, updated)
VALUES
('619abdbf-41bd-4f2a-a482-dcbd8151fe6d', 'TEST', 0, 'm', 1, now(), now()),
('629abdbf-41bd-4f2a-a482-dcbd8151fe6d', 'TEST2', 0, 'm', 2, now(), now()),
('639abdbf-41bd-4f2a-a482-dcbd8151fe6d', 'TEST3', 0, 'f', 3, now(), now()),
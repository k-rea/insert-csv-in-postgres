CREATE TABLE public.persons
(
    family_name character varying(256),
    first_name character varying(256) DEFAULT ''::character varying NOT NULL,
    age integer NOT NULL
);

ALTER TABLE public.persons
OWNER TO postgres;
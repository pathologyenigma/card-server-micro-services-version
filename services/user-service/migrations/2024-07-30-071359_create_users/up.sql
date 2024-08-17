-- Your SQL goes here
CREATE TABLE public.users
(
    id uuid NOT NULL,
    name character varying(16) NOT NULL,
    email character varying(56) NOT NULL,
    password character varying NOT NULL,
    image text,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    PRIMARY KEY (id),
    CONSTRAINT unique_email_should_be_unique UNIQUE (email)
);

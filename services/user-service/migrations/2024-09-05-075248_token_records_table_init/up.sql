-- Your SQL goes here
CREATE TYPE public.invalid_token_reason AS ENUM
    ('BLOCKED', 'ALREADYLOGOUT');
CREATE TABLE public.invalid_token_records
(
    id uuid NOT NULL,
    token_value text NOT NULL,
    invalid_reason invalid_token_reason NOT NULL,
    PRIMARY KEY (id)
);
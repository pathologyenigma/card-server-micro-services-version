-- Your SQL goes here
CREATE TABLE public.friend_requests
(
    id uuid NOT NULL,
    "from" uuid NOT NULL,
    "to" uuid NOT NULL,
    is_accepted boolean NOT NULL DEFAULT false,
    sent_at timestamp without time zone NOT NULL,
    accepted_at timestamp without time zone,
    PRIMARY KEY (id),
    CONSTRAINT fk_from_user_id FOREIGN KEY ("from")
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT fk_to_user_id FOREIGN KEY ("to")
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
        NOT VALID
);
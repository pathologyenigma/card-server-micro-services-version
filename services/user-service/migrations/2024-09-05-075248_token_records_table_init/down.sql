-- This file should undo anything in `up.sql`
drop table public.token_records if exists;
drop type public.invalid_token_reason if exists;
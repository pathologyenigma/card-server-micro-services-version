-- This file should undo anything in `up.sql`
ALTER TABLE IF EXISTS public.users
    DROP COLUMN description text if exists;
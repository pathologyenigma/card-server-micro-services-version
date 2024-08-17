-- This file should undo anything in `up.sql`
ALTER TABLE friend_requests DROP CONSTRAINT fk_from_user_id;
ALTER TABLE friend_requests DROP CONSTRAINT fk_to_user_id;
drop TABLE friend_requests;
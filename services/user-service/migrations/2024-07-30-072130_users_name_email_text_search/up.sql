-- Your SQL goes here
create index users_name_email_text_search_idx on users using gin(to_tsvector('english', name ||'' || email));
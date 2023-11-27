ALTER TABLE todos drop column created_at;
alter table todos add column created_at timestamp with time zone default now();

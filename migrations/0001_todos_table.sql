create table todos (
	id serial PRIMARY KEY not null,
  name varchar not null,
  discription varchar not null
);

create unique index todo_id_idx on todos (id);


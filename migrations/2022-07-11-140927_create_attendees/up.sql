CREATE TABLE attendees (
  id integer primary key autoincrement not null,
  hash char(64) not null,
  preimage char(64) not null,
  firstname varchar(100) not null,
  lastname varchar(100) not null,
  email varchar(50) not null,
  data1 varchar(200) not null,
  paid boolean default false not null,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP not null
)
create table watches
(
    -- without not null, sqlx will infer id as nullable
    id         integer primary key autoincrement not null,
    path       text                              not null,
    status     text                              not null,
    created_at datetime                          not null default current_timestamp
);

create unique index watches_path_idx on watches (path);


create table documents
(
    id         integer primary key autoincrement not null,
    path       text                              not null,
    watch_id   integer                           not null references watches (id),
    created_at datetime                          not null default current_timestamp,
    indexed_at datetime
);

create unique index documents_path_idx on documents (path);


create table jobs
(
    id         integer primary key autoincrement not null,
    job_type   text                              not null,
    watch_id   integer                           not null references watches (id) on delete cascade,
    status     text                              not null,
    created_at datetime                          not null,
    started_at datetime                          not null
);

create index jobs_watch_id_idx on jobs (watch_id);

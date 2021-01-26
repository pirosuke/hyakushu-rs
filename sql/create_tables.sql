drop table if exists wakas;
create table wakas(
    waka_id int not null
    ,kamino_ku varchar(100) not null
    ,shimono_ku varchar(100) not null
    ,yomi_bito varchar(100) not null default ''
    ,primary key(waka_id)
)
;

drop table if exists players;
create table players(
    player_id serial not null
    ,name varchar(100) not null
    ,primary key(player_id)
)
;

drop table if exists question_sets;
create table question_sets(
    question_set_id serial not null
    ,player_id int not null
    ,waka_id_list jsonb not null
    ,insert_datetime timestamp with time zone not null default now()
    ,primary key(question_set_id)
)
;
drop table if exists answer_logs;
create table answer_logs(
    log_id serial not null
    ,player_id int not null
    ,question_set_id int not null
    ,waka_id int not null
    ,answered_correctly bool not null
    ,insert_datetime timestamp with time zone not null default now()
    ,primary key(log_id)
)
;

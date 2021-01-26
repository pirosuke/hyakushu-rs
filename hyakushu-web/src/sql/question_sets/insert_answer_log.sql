insert into answer_logs(
    player_id
    ,question_set_id
    ,waka_id
    ,answered_correctly
)
values
(
    $1
    ,$2
    ,$3
    ,$4
)
returning log_id

insert into question_sets(
    player_id
    ,waka_id_list
)
values
(
    $1
    ,$2
)
returning question_set_id

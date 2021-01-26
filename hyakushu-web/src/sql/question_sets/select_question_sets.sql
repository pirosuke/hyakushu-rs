select
    question_set_id
    ,player_id
    ,waka_id_list
    ,insert_datetime
from question_sets
where player_id = $1
order by insert_datetime desc
offset $2 limit $3

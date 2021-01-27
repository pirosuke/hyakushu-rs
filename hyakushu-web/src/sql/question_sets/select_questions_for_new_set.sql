with
mostly_misanswered_questions as (
    select
        waka_id
        ,count(*) as count
    from answer_logs
    where player_id = $1
    and not answered_correctly
    group by waka_id
    order by count desc
    limit ($2 / 2)::int
)
,random_questions as (
    select
        waka_id
    from wakas
    order by random()
    limit $2
)
select
    waka_id
    ,kamino_ku
    ,shimono_ku
    ,yomi_bito
from wakas
where waka_id in (
    select
        waka_id
    from mostly_misanswered_questions
    union
    select
        waka_id
    from random_questions
)
limit $2

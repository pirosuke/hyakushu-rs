with
correct_answer_rates as (
    select
        waka_id
        ,count(*) filter (where answered_correctly) as correct_count
        ,count(*) as total_count
        ,(count(*) filter (where answered_correctly)::float) / count(*)::float as rate
    from answer_logs
    where player_id = $1
    group by waka_id
)
,wakas_by_answer_rate as (
    select
        w.waka_id
        ,coalesce(car.correct_count, 0)::float / coalesce(total_count, 1)::float as rate
    from wakas w
    left join correct_answer_rates car
        on w.waka_id = car.waka_id
    order by rate asc
    limit ($2 / 2)::int
)
,random_wakas as (
    select
        waka_id
    from wakas
    where waka_id not in (
        select
            waka_id
        from wakas_by_answer_rate
    )
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
    from wakas_by_answer_rate
    union
    select
        waka_id
    from random_wakas
)
limit $2

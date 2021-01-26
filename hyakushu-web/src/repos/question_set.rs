extern crate serde_json;

use serde::{Deserialize, Serialize};
use r2d2_postgres::{postgres, PostgresConnectionManager, r2d2::PooledConnection};
use rand::prelude::*;

use crate::repos::waka::{Waka, fetch_wakas_all};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub waka: Waka,
    pub answer_choices: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionSet {
    question_set_id: i32,
    player_id: i32,
    questions: Vec<Question>,
}

pub fn generate_question_set(player_id: i32, num_of_questions: i64, pg_client: &mut PooledConnection<PostgresConnectionManager<postgres::NoTls>>) -> QuestionSet {
    // wakasからnum_of_questions分のwakaを取得する
    // その際、自分の最近の解答回数が少ない質問、間違いが多い回答を優先して取得する
    let select_question_sql = include_str!("../sql/question_sets/select_questions_for_new_set.sql");

    let mut question_waka_list: Vec<Waka> = Vec::new();
    for row in pg_client.query(select_question_sql, &[
        &player_id,
        &num_of_questions,
    ]).unwrap() {
        question_waka_list.push(Waka{
            waka_id: row.get("waka_id"),
            kamino_ku: row.get("kamino_ku"),
            shimono_ku: row.get("shimono_ku"),
            yomi_bito: row.get("yomi_bito"),
        });
    }

    // 作成した問題を保存する
    let insert_question_set_sql = include_str!("../sql/question_sets/insert_question_set.sql");
    let waka_id_list: Vec<i32> = question_waka_list.iter()
        .map(|waka| waka.waka_id)
        .collect();
    let waka_id_list_json: serde_json::Value = serde_json::from_str(serde_json::to_string(&waka_id_list).unwrap().as_str()).unwrap();

    let row = pg_client.query_one(insert_question_set_sql, &[
        &player_id,
        &waka_id_list_json,
    ]).unwrap();

    let question_set_id = row.get("question_set_id");

    // 正解とは別でランダムに3つの下の句を取得して選択肢とする
    let mut choice_waka_list = fetch_wakas_all(pg_client);

    let mut rng = rand::thread_rng();
    let mut question_list: Vec<Question> = Vec::new();
    for question_waka in question_waka_list {
        choice_waka_list.shuffle(&mut rng);
        let mut answer_choices: Vec<String> = choice_waka_list.iter()
            .filter(|waka| waka.shimono_ku != question_waka.shimono_ku)
            .take(3)
            .map(|waka| waka.shimono_ku.clone())
            .collect();
        answer_choices.push(question_waka.shimono_ku.clone());
        answer_choices.shuffle(&mut rng);
        let q = Question{
            waka: question_waka.clone(),
            answer_choices: answer_choices,
        };
        question_list.push(q);
    }

    QuestionSet{
        question_set_id: question_set_id,
        player_id: player_id,
        questions: question_list,
    }
}

pub fn add_answer_log(player_id: i32, question_set_id: i32, waka_id: i32, answered_correctly: bool, pg_client: &mut PooledConnection<PostgresConnectionManager<postgres::NoTls>>) {
    let insert_answer_log_sql = include_str!("../sql/question_sets/insert_answer_log.sql");

    pg_client.query_one(insert_answer_log_sql, &[
        &player_id,
        &question_set_id,
        &waka_id,
        &answered_correctly,
    ]).unwrap();

}
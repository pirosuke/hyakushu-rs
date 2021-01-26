extern crate serde;
extern crate serde_json;

use actix_web::{HttpResponse, web, post};
use log::{info};
use serde::{Deserialize};
use r2d2_postgres::{postgres, r2d2::Pool, PostgresConnectionManager};

use crate::routes::{JSNumberType};

use crate::repos::question_set;

#[derive(Deserialize)]
pub struct QuestionSetAddForm {
    player_id: JSNumberType,
}

#[derive(Deserialize)]
pub struct AnswerLogAddForm {
    player_id: JSNumberType,
    question_set_id: JSNumberType,
    waka_id: JSNumberType,
    answered_correctly: bool,
}

#[post("/generate")]
pub async fn question_set_generate_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<QuestionSetAddForm>) -> HttpResponse {
    info!("start question_set_generate_handler");

    let player_id = match params.player_id.clone() {
        JSNumberType::Float(v) => v as i32,
        JSNumberType::Str(v) => v.unwrap_or("0".to_string()).parse::<i32>().unwrap(),
    };

    let mut pg_client = pg_pool.get().unwrap();
    let question_set = question_set::generate_question_set(player_id, 10, &mut pg_client);

    HttpResponse::Ok().json(question_set)
}

#[post("/add_answer")]
pub async fn add_answer_log_handler(pg_pool: web::Data<Pool<PostgresConnectionManager<postgres::NoTls>>>, params: web::Json<AnswerLogAddForm>) -> HttpResponse {
    info!("start add_answer_log_handler");

    let player_id = match params.player_id.clone() {
        JSNumberType::Float(v) => v as i32,
        JSNumberType::Str(v) => v.unwrap_or("0".to_string()).parse::<i32>().unwrap(),
    };

    let question_set_id = match params.question_set_id.clone() {
        JSNumberType::Float(v) => v as i32,
        JSNumberType::Str(v) => v.unwrap_or("0".to_string()).parse::<i32>().unwrap(),
    };

    let waka_id = match params.waka_id.clone() {
        JSNumberType::Float(v) => v as i32,
        JSNumberType::Str(v) => v.unwrap_or("0".to_string()).parse::<i32>().unwrap(),
    };

    let answered_correctly = params.answered_correctly;

    let mut pg_client = pg_pool.get().unwrap();
    question_set::add_answer_log(player_id, question_set_id, waka_id, answered_correctly, &mut pg_client);

    HttpResponse::Ok().finish()
}

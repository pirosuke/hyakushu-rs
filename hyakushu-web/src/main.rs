extern crate clap;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader};

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Deserialize};
use r2d2_postgres::{postgres, r2d2::Pool, PostgresConnectionManager};

mod routes;
mod repos;

use routes::question_sets;

#[derive(Deserialize)]
struct DbConfig {
    host: String,
    port: String,
    db_name: String,
    user_name: String,
    password: String,
}

#[derive(Deserialize)]
struct AppConfig {
    public_dir_path: String,
    server_host: String,
    db: DbConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli_options = clap::App::new("hyakushu-web")
        .about("Hyakushu Web")
        .arg(clap::Arg::with_name("config")
            .help("Config File Path")
            .long("config")
            .short("c")
            .takes_value(true)
        )
        .get_matches();
    
    let p_config_file_path = cli_options.value_of("config").unwrap();
    let config_file_path = Path::new(p_config_file_path);
    if !config_file_path.exists() {
        panic!("Config file not found");
    }

    let config_file = match File::open(config_file_path) {
        Ok(n) => n,
        Err(err) => panic!("Config file opening error: {:?}", err),
    };

    let app_config: AppConfig = serde_json::from_reader(BufReader::new(config_file)).unwrap();

    let manager = PostgresConnectionManager::new(
        format!(
            "postgresql://{user}:{password}@{host}:{port}/{db_name}",
            user=app_config.db.user_name,
            password=app_config.db.password,
            host=app_config.db.host,
            port=app_config.db.port,
            db_name=app_config.db.db_name,
        ).parse().unwrap(),
        postgres::NoTls,
    );
    let pg_pool = Pool::new(manager).unwrap();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let public_dir_path = app_config.public_dir_path.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pg_pool.clone())
            .service(
                web::scope("/question_sets")
                    .service(question_sets::question_set_generate_handler)
                    .service(question_sets::add_answer_log_handler)
            )
            .service(fs::Files::new("/", public_dir_path.clone()).show_files_listing())
    })
    .bind(app_config.server_host)?
    .run()
    .await
}
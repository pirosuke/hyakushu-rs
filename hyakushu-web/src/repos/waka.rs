use serde::{Deserialize, Serialize};
use r2d2_postgres::{postgres, PostgresConnectionManager, r2d2::PooledConnection};


#[derive(Clone, Serialize, Deserialize)]
pub struct Waka {
    pub waka_id: i32,
    pub kamino_ku: String,
    pub shimono_ku: String,
    pub yomi_bito: String,
}

pub fn fetch_wakas_all(pg_client: &mut PooledConnection<PostgresConnectionManager<postgres::NoTls>>) -> Vec<Waka> {
    let sql = include_str!("../sql/wakas/select_wakas.sql");

    let mut waka_list: Vec<Waka> = Vec::new();
    for row in pg_client.query(sql, &[]).unwrap() {
        waka_list.push(Waka{
            waka_id: row.get("waka_id"),
            kamino_ku: row.get("kamino_ku"),
            shimono_ku: row.get("shimono_ku"),
            yomi_bito: row.get("yomi_bito"),
        });
    }

    waka_list
}
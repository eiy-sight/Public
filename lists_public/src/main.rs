#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use sqlx::{query, FromRow, Row};
use std::{error::Error, result};

#[derive(Debug, FromRow)]
struct Thing {
    pub index: i64,
    pub item: String,
    pub amount: f64,
    pub notes: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/anima";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    let name = String::from("gradient-borders");
    let value = String::from("false");

    options(name, value, &pool).await?;

    Ok(())
}

async fn appearance(
    station: String,
    part: String,
    value: (i16, String),
    pool: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> {
    // #000000ff


    let query = format!("SELECT {} FROM appearance WHERE station = '{}'", part, station);
    let query_results = sqlx::query(&query)
        .fetch_one(pool)
        .await?;

    let mut hex: String = query_results.get(part.as_str());
    println!("{}", hex);

    match value.0 {
        0 => hex.replace_range(1..3, &value.1),
        1 => hex.replace_range(3..5, &value.1),
        2 => hex.replace_range(5..7, &value.1),
        3 => hex.replace_range(7..9, &value.1),
        _ => ()
    };


    let query = format!("UPDATE appearance SET {} = '{}' WHERE {} = '{}'", part, hex, "station", station);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    println!("{}", hex);

    Ok(())
}

async fn options(
    name: String,
    value: String,
    pool: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> { 
    let query = format!("SELECT option FROM options WHERE name = '{}'", name);
    let query_results = sqlx::query(&query)
        .fetch_one(pool)
        .await?;

    let boolean: bool = query_results.get("option");

    let new_value = !boolean;

    let query = format!("UPDATE options SET option = {} WHERE name = '{}'", new_value, name);
    sqlx::query(&query)
        .execute(pool)
        .await?;

    Ok(())
}

async fn insert(
    table: &str,
    item: &str,
    amount: &f64,
    notes: &str,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    let insert_quesry = format!(
        "INSERT INTO {}(index, item, amount, notes) VALUES (NEXTVAL('increment_seq'), $2, $3, $4)",
        table
    );

    sqlx::query(&insert_quesry)
        .bind(table)
        .bind(item)
        .bind(amount)
        .bind(notes)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(
    name: &str,
    all: bool,
    primary_column: Option<&str>,
    where_clause: Option<&str>,
    where_value: Option<&str>,
    pool: &sqlx::PgPool,
) -> Result<Vec<Thing>, Box<dyn Error>> {
    let where_bool: bool;

    match where_clause {
        Some(t) => where_bool = true,
        None => where_bool = false,
    }

    if all {
        let query = format!("SELECT * FROM {}", name);
        let query_result = sqlx::query_as::<_, Thing>(&query)
            .fetch_all(pool)
            .await?;

        Ok(query_result)
    } else {
        if where_bool {
            let query = format!("SELECT $1 FROM {} WHERE $2 = $3", name);
            let query_result = sqlx::query(&query)
                .bind(primary_column)
                .bind(where_clause)
                .bind(where_value)
                .fetch_all(pool)
                .await?;

            let result: Vec<Thing> = query_result
                .iter()
                .map(|row| {
                    println!("{:?}", row);
                    Thing {
                        index: row.get("index"),
                        item: row.get("item"),
                        amount: row.get("amount"),
                        notes: row.get("notes"),
                    }
                })
                .collect();

            Ok(result)
        } else {
            let query = format!("SELECT $1 FROM {}", name);

            let query_result = sqlx::query(&query)
                .bind(primary_column)
                .fetch_all(pool)
                .await?;

            let result: Vec<Thing> = query_result
                .iter()
                .map(|row| {
                    println!("{:?}", row);
                    Thing {
                        index: row.get("index"),
                        item: row.get("item"),
                        amount: row.get("amount"),
                        notes: row.get("notes"),
                    }
                })
                .collect();

            Ok(result)
        }
    }
}

async fn delete(index: &i32, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn update(
    table: &str,
    item: &str,
    amount: &i32,
    notes: &str,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

async fn create_table(name: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let table_query = format!("CREATE TABLE {}(index BIGINT NOT NULL, item VARCHAR(255) NOT NULL, amount FLOAT8 NOT NULL, notes TEXT)", name);
    sqlx::query(&table_query).execute(pool).await?;

    Ok(())
}

async fn delete_table(name: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let delete_query = format!("DROP TABLE {}", name);

    sqlx::query(&delete_query).execute(pool).await?;

    Ok(())
}

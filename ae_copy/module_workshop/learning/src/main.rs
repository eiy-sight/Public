#![allow(non_snake_case)]

use std::{thread, time};
use rand::Rng;

#[derive(Clone)]
pub struct QAndA {
    pub id: i32,
    pub question: String,
    pub answer: String,
}

#[tokio::main]
async fn main() {
    //                                  [name]:[password]@[ip]:[port]/[database_name]
    let url = "postgres://i:ae@192.168.0.120:5432/anima_enterprises";
    let pool = sqlx::postgres::PgPool::connect(url)
        .await
        .expect("pooling error");
    loop {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("failed to read command");
        let separated = command.split(" // ").collect::<Vec<&str>>();

        match separated[0].trim() {
            "clear" => {
                break;
            }

            // help
            "h" => {
                println!("'clear' for exit");
                println!("'a' for showing entire table");
                println!("'c // [question] // [answer]' for creating new card");
                println!("'d // [id]' for deleting id'd card");
                println!("'u // [id] // [new question] // [new answer]' for updating card");
                println!("'r' for showing random card");
                println!("'u // [id]' for showing specific card");

            }

            // shows all
            "a" => {
                let all = read_all(&pool).await;
                for entry in all {
                    println!("{} | {} | {}", entry.id, entry.question, entry.answer);
                }
            }

            // c // what is mayas true name // etheria
            "c" => {
                println!("Creating new card");
                insert(separated[1].trim(), separated[2].trim(), &pool)
                    .await
                    .expect("Failed to insert new card");
            }
            // d // 1
            "d" => {
                println!("Deleting card");
                delete(
                    &separated[1]
                        .trim()
                        .parse::<i32>()
                        .expect("failed to parse delete int"),
                    &pool,
                )
                .await
                .expect("Failed to delete card");
            }
            // u // 1 // what is mayas true name // etheria
            "u" => {
                println!("Updating card");
                update(
                    &separated[1]
                        .trim()
                        .parse::<i32>()
                        .expect("failed to parse update int"),
                    separated[2].trim(),
                    separated[3].trim(),
                    &pool,
                )
                .await
                .expect("Failed to update card");
            }
            // r // 1
            "r" => {
                match separated.get(1) {
                    Some(value) => {
                        let index = value.to_string().trim().parse::<i32>().unwrap();
                        let read_value = read(false, index, &pool).await;
                        let mut _wait = String::new();
                        println!("{}", read_value.question);

                        std::io::stdin()
                            .read_line(&mut command)
                            .expect("failed to read command");

                        println!("{}", read_value.answer);
                    }
                    None => {
                        let read_value = read(true, 0, &pool).await;
                        println!("{}", read_value.question);

                        thread::sleep(time::Duration::from_secs(120));

                        println!("{}", read_value.answer);
                    }
                };
            }
            _ => {}
        }
    }

    // pipe setup will make better in the future by setting pipe path to a learning local path
    // let pipe_path = "/tmp/learning_pipe";
    // match Command::new("mkfifo").arg(pipe_path).status() {
    //     Ok(_) => {}
    //     Err(_) => {}
    // };

    // let mut cur_hour: i8 = Local::now().format("%H").to_string().parse().unwrap();
    // loop {
    //     // check the time
    //     let now = Local::now();

    //     thread::sleep(time::Duration::from_secs(2));
    //     if (now.format("%M").to_string() == "56")
    //         && cur_hour == now.format("%H").to_string().parse::<i8>().unwrap()
    //     {
    //         // sets up the terminal and has it listen to pipe path
    //         Command::new("gnome-terminal")
    //             .arg("--")
    //             .arg("bash")
    //             .arg("-c")
    //             .arg(format!("tail -f {}", pipe_path))
    //             .spawn()
    //             .expect("failed");

    //         let mut pipe = OpenOptions::new()
    //             .write(true)
    //             .open(pipe_path)
    //             .expect("failed to open write path");

    //         // here i need to get the question from a database and write it to the pipe

    //         // after a time display the answer

    //         writeln!(pipe, "{}", now.format("%H%M")).unwrap();
    //         cur_hour = Local::now().format("%H").to_string().parse().unwrap();
    //         cur_hour += 1;
    //     }
    // }
}

// fn get_info() {}

// // table format: question answer as TEXT
async fn insert(question: &str, answer: &str, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let insert_query = "INSERT INTO study (question, answer) VALUES($1, $2)";

    sqlx::query(&insert_query)
        .bind(question)
        .bind(answer)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read_all(pool: &sqlx::PgPool) -> Vec<QAndA> {
    let info = sqlx::query_as!(QAndA, "SELECT * FROM study")
        .fetch_all(pool)
        .await
        .expect("Failed to get all");

    info
}

async fn read(rand: bool, index: i32, pool: &sqlx::PgPool) -> QAndA {
    let info = sqlx::query_as!(QAndA, "SELECT id, question, answer FROM study",)
        .fetch_all(pool)
        .await
        .expect("");

    let valid_ids: Vec<i32> = info.clone().into_iter().map(|valid| valid.id).collect();

    if rand {
        let count = info.len();
        let mut rng = rand::thread_rng();
        let chosen = rng.gen_range(0..count);
        let chosen_id = valid_ids[chosen];
        let chosen_QAndA = info.clone().into_iter().find(|value| value.id == chosen_id);
        match chosen_QAndA {
            Some(value) => return value,
            None => return info[0].clone(),
        }
    } else {
        let chosen_QAndA = info.clone().into_iter().find(|value| value.id == index);
        match chosen_QAndA {
            Some(value) => return value,
            None => return info[0].clone(),
        }
    }
}

async fn delete(index: &i32, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let delete_query = "DELETE FROM study WHERE id = $1";
    sqlx::query(&delete_query).bind(index).execute(pool).await?;

    Ok(())
}

async fn update(
    index: &i32,
    question: &str,
    answer: &str,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    let update_query = "UPDATE study SET question = $1, answer = $2 WHERE id = $3";

    sqlx::query(&update_query)
        .bind(question)
        .bind(answer)
        .bind(index)
        .execute(pool)
        .await?;

    Ok(())
}

use sqlx::mysql::{MySql, MySqlPoolOptions};
use sqlx::Pool;
use crate::task::Task;
pub async fn init_pool() -> Pool<MySql> {
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@localhost:3306/test")
        .await {
            Ok(pool) => pool,
            Err(_) => panic!("连接数据库失败"),
        };
    pool
}

pub async fn get_all_tasks(pool: &Pool<MySql>) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(tasks)
}

pub async fn add_task(pool: &Pool<MySql>, name: String) -> Result<bool, sqlx::Error>{
    sqlx::query("INSERT INTO todos (name, completed) VALUES (?, false)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(true)
}

pub async fn set_completed_task(pool: &Pool<MySql>, name: String, completed: bool) -> Result<bool, sqlx::Error> {
    sqlx::query("UPDATE todos SET completed = ? WHERE name = ?")
        .bind(completed)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(true)
}
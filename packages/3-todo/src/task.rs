use colored::*;
use sqlx::{Pool, MySql};
use crate::sql::{get_all_tasks, set_completed_task, add_task as add_task_sql};
// Task 结构体，包含 name 和 completed 两个字段
#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct Task {
  name: String,
  completed: bool,
}
// Task 结构体方法的实现
impl Task {
  #[allow(dead_code)]
  // new 方法，用于创建 Task 实例, 约定俗成 new 是一个构造器（new 是关键字，不是方法）
  pub fn new(name: String) -> Self {
    Self {
      name,
      completed: false,
    }
  }
}

// 增加任务
pub async fn add_task(sql_pool: &Pool<MySql>, name: String) {
  match add_task_sql(sql_pool, name).await {
    Ok(_) => println!("{}", "添加成功".green()),
    Err(_) => println!("{}", "添加失败".red()),
  };
}
// 标记任务完成
pub async fn complete_task(sql_pool: &Pool<MySql>, index: usize) {
  // let task = tasks.get(index).unwrap();
  let tasks = get_all_tasks(sql_pool).await.unwrap();
  let task = tasks.get(index).unwrap();
  // tasks[index].completed = true;
  match set_completed_task(sql_pool, task.name.to_string(), true).await {
    Ok(_) => println!("{}", "标记成功".green()),
    Err(_) => println!("{}", "标记失败".red()),
  };
}

// 取消任务完成标记
pub async fn uncomplete_task(sql_pool: &Pool<MySql>, index: usize) {
  let tasks = get_all_tasks(sql_pool).await.unwrap();
  let task = tasks.get(index).unwrap();
  match set_completed_task(sql_pool, task.name.to_string(), false).await {
    Ok(_) => println!("{}", "取消标记成功".green()),
    Err(_) => println!("{}", "取消标记失败".red()),
  };
}

// 展示任务列表
pub async fn show_tasks(sql_pool: &Pool<MySql>) {
  let tasks = get_all_tasks(sql_pool).await.unwrap();
  for (index, task) in tasks.iter().enumerate() {
    let task_status = if task.completed {
      "已完成".green()
    } else {
      "未完成".red()
    };
    println!("{}: {} {}", index + 1, task.name, task_status);
  }
}
mod task;
mod sql;

use task::{ add_task, complete_task, uncomplete_task, show_tasks};
use sql::init_pool;
use std::io;
use colored::*;
#[tokio::main]
async fn main() {
    // 创建一个空的任务列表
    let sql_pool = init_pool().await;
    // 告知用户可以输入的命令
    println!("{}", "1: 增加任务".green());
    println!("{}", "2: 标记任务完成".green());
    println!("{}", "3: 取消任务完成标记".green());
    println!("{}", "4: 查看所有任务".green());
    println!("{}", "5: 退出".blue());
    println!("=======开始=========");
    show_tasks(&sql_pool).await;
    println!("{}", "请输入命令序号：".green());
    loop {
      let user_input = get_user_input::<i32>();
      match user_input {
          1 => {
              println!("{}", "请输入任务名称".green());
              let task_name = get_user_input::<String>();
              add_task(&sql_pool, task_name).await;
          },
          2 => {
            println!("{}", "请输入需要完成任务序号".green());
            let task_num = get_user_input::<usize>();
            complete_task(&sql_pool, task_num - 1).await;
          },
          3 => {
            println!("{}", "请输入需要重置为未完成的任务序号".green());
            let task_num = get_user_input::<usize>();
            uncomplete_task(&sql_pool, task_num - 1).await;
          },
          4 => {
            show_tasks(&sql_pool).await;
          },
          5 => {
            println!("{}", "退出程序".red());
            break;
          },
          _ => {
              println!("{}", "请输入正确的数字，数字范围1-4".red());
          }
      }
      println!("{}", "请继续输入命令序号：".green());
    }
}

fn get_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("无法读取输入");
    user_input.trim().parse().expect("无法解析输入")
}
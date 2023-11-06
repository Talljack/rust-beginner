use std::error::Error;
use std::fs;
use colored::*;

pub struct Config {
  query: String,
  filename: String,
  case_sensitive: bool,
}


impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments".into());
    }
    // 第一个参数是程序名，所以跳过
    // args 本身是一个迭代器，所以可以直接调用 next 方法
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string".into()),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name".into()),
    };

    let case_sensitive = match args.next() {
      Some(arg) => {
        if arg == "true" {
          true
        } else {
          false
        }
      },
      None => false,
    };
    return Ok(Config { query, filename, case_sensitive });
  }
}


pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(query)).collect()
}

// 大小写不敏感，直接转换为小写再查询
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str())).collect()
}

pub fn grep_text(config: &Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)?;
  let grep_result = if config.case_sensitive {
    search_case_sensitive(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in grep_result {
    println!("{}", line.green());
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search_case_sensitive(query, contents)
    );
  }
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
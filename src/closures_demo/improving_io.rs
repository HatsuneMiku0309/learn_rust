use std::{env, process};

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("引數不足");
    //     }

    //     let query = args[1].clone();
    //     let filename = args[2].clone();

    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
    //     Ok(Config {
    //         query,
    //         filename,
    //         case_sensitive
    //     })
    // }

    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("沒有取得欲搜尋的字串")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("沒有取得檔案名稱"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

pub fn set_args_in_config() -> Config {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題: {}", err);
        process::exit(1);
    });

    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("解析引數時出現問題: {}", err);
    //     process::exit(1);
    // });

    config
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    /* 
        疊代配接器（iterator adaptor）
        疊代器是 Rust 其中一種零成本抽象（zero-cost abstractions）
        這是 C++ 的初始設計暨實作者 Bjarne Stroustrup 在 “Foundations of C++” (2012) 書中所定義的零開銷（zero-overhead）的概念:
            大致上來說，C++ 的實作遵守著零開銷的原則：你沒有使用到的話，你就不必買單。而且你有使用到的話，你不可能再寫出更好的程式碼。
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
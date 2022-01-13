use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // 异常类型的返回值是一个全局的字符串字面值
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = (&args[1]).clone();
        let filename = (&args[2]).clone();
        // 如果环境变量有CASE_INSENSITIVE，is_err()返回false，即不区分大小写；
        // 如果环境变量没有CASE_INSENSITIVE，is_err()返回true，即区分大小写
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }

    pub fn new2(mut args: env::Args) -> Result<Config, &'static str> {
        // 第一个参数是程序名，需要忽略
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})
    }
}

// 返回值是一个实现了Error trait的对象，dyn表示是动态的，根据不同的错误类型可以绑定不同的Error trait
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // 通过？表达式来获取成功时的值，如果错误的话会直接返回Error相关值
    let contents = fs::read_to_string(&config.filename)?;
    let result= if config.case_sensitive {
            search_v2(&config.query, &contents)
        } else {
            search_case_insensitive_v2(&config.query, &contents)
        };
    for re in result {
        println!("{}", re);
    }
    Ok(())
}

// 函数签名使用了生命周期'a,说明返回值的str和入参contents的生命周期相同，
// 因为返回的切片应该是入参contents的切片
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_v2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

// 区分大小写的search匹配
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive_v2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&(query.to_lowercase()))
    }).collect()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
Rust:\n
safe, fast, productive.\n
Pick three\n\
Duct page.";
        assert_eq!(vec!["safe, fast, productive."],
        // search(query, contents));
        search_v2(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rUst";
        let contents = "\
Rust:\n
safe, fast, productive.\n
Pick three.\n\
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents));
    }
}
extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

// 使用方式：cargo run test poem.txt
// 设置环境变量CASE_INSENSITIVE
// export CASE_INSENSITIVE=1，通过echo $CASE_INSENSITIVE进行查看

fn main() {
    // let args: Vec<String> = env::args().collect();
    // 使用闭包，闭包参数放在||中，可以在后续进行调用
    // let config = Config::new(&args).unwrap_or_else(|err|{
    //     // eprintln!宏向标准错误打印信息
    //     eprintln!("Problem parsing arguments {}", err);
    //     process::exit(1);
    // });
    let config = Config::new2(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });
    // 测试环境调试的输出
    println!("query : {}", config.query);
    println!("filename: {}", config.filename);
    // 因为run成功时无返回值，因此使用if let仅对错误时进行处理
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error, {}", e);
        process::exit(1);
    }

}

// 把输出信息重定向到output.txt => cargo run > output.txt 或者 cargo run to poem.txt > output.txt
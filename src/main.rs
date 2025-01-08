use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Welcome to my shell!");

    loop {
        // 打印提示符
        print!("# ");
        io::stdout().flush().unwrap();

        // 读取用户输入
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // 去除末尾的换行符
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // 将输入分割成命令和参数
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        // 特殊处理 cd 命令
        if command == "cd" {
            if let Some(dir) = args.first() {
                match std::env::set_current_dir(dir) {
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("Failed to change directory: {}", e);
                        continue;
                    }
                }
            } else {
                eprintln!("cd: missing directory argument");
                continue;
            }
        }

        // 在处理命令之前添加
        if command == "exit" || command == "quit" {
            println!("Goodbye!");
            break; // 退出 loop
        }

        if command == "pwd" {
            if let Ok(current_dir) = std::env::current_dir() {
                println!("{}", current_dir.display());
                continue;
            }
        }

        match command {
            "echo" => {
                // 处理环境变量展开，比如 echo $HOME
                for arg in args {
                    if arg.starts_with('$') {
                        let var_name = &arg[1..];
                        if let Ok(value) = std::env::var(var_name) {
                            print!("{} ", value);
                        }
                    } else {
                        print!("{} ", arg);
                    }
                }
                println!();
                continue;
            }
            _ => {
                // 创建并执行命令
                match Command::new(command).args(&args).spawn() {
                    Ok(mut child) => {
                        // 等待子进程完成
                        match child.wait() {
                            Ok(status) => {
                                if !status.success() {
                                    eprintln!("Command executed with failing error code");
                                }
                            }
                            Err(e) => eprintln!("Error waiting for command: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Execution failed: {}", e),
                }
            }
        }
    }
}

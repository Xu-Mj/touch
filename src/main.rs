use std::{fs::File, io::{self, Write}, path::Path};

use clap::{arg, command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .version("0.1.0")
        .author("Xu-Mj")
        .about("`touch` command on windows") 
        .arg(Arg::new("filename").action(ArgAction::Append))
        .arg(arg!(-f --force "force overwrite").action(ArgAction::SetTrue))
        .get_matches();

    let args = matches
        .get_many::<String>("filename")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    let mut force = false;
    if matches.get_flag("force") {
        force = true;
    }
    if args.is_empty() {
        println!("[INFO]: `touch` needs at least one filename...");
    } else {
        // 循环创建文件
        for name in args {
            if !force {
                if Path::new(name).exists() {
                    print!("[WARN]: file {name} exists; do you want to overwrite? y/n ");
                    // get user input
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "y" {
                        println!("[INFO]: skip file {name}");
                        continue;
                    }
                }
            }
            if let Err(err) = File::create(name) {
                println!("[ERROR]: {err}");
                continue;
            }
        }
    }
}

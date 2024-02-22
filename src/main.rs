use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use clap::{arg, command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .version("0.1.0")
        .author("Xu-Mj")
        .about("`touch` command on windows")
        .arg(Arg::new("filename").action(ArgAction::Append))
        .arg(arg!(-f --force "force overwrite").action(ArgAction::SetTrue))
        .arg(arg!(-i --install "installation").action(ArgAction::SetTrue))
        .get_matches();

    let args = matches
        .get_many::<String>("filename")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    if matches.get_flag("install") {
        // install to PATH
        return install();
    }

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

fn install() {
    println!("[INFO]: `touch` is installing...");
    // get current dir
    match std::env::current_dir() {
        Ok(path) => {
            let path = path.display();
            println!("[INFO]: install path: {}", path);
            let command = format!(
                r#"
                $addPath='{path}';
                $target='Machine'; 
                $path = [Environment]::GetEnvironmentVariable('Path', $target); 
                if($path -match ";$")
                    {{ $newPath = $path + $addPath; }} 
                else 
                    {{ $newPath = $path + ';' + $addPath; }} 
                [Environment]::SetEnvironmentVariable('Path', $newPath, $target)"#
            );
            // add path to evironment variable
            println!("[INFO]: add current dir to environment variable ...");
            match std::process::Command::new("powershell.exe")
                .arg(command)
                .output()
            {
                Ok(res) => {
                    // run command success
                    if res.status.success() {
                        println!("[INFO]: installation success!");
                        println!("[INFO]: congratulations!");
                        println!("[INFO]: please restart your terminal");
                    } else {
                        // run command failed
                        println!("[ERROR]: execute installation command failed!");
                        println!("[INFO]: maybe you should run `touch -i/--install` with administrator privilege");
                    }
                }
                Err(_) => {
                    println!("[ERROR]: execute installation command failed!");
                    println!("[INFO]: maybe you should run `touch -i/--install` with administrator privilege");
                }
            }
        }
        Err(err) => {
            println!("[ERROR]: get current dir failed: {err}");
        }
    };
}

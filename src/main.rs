use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::Path,
};

use hsh::hsh_cmds::{Flag, Flags, HshCmds};

fn main() {
    let mut vars: HashMap<String, String> = HashMap::new();
    loop {
        print!("hsh > ");

        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line ...");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let mut cmd_flags: Flags = Flags::default();
        let args: Vec<String> = parts.filter_map(|s| resolve_variable(s, &vars, &mut cmd_flags)).collect();

        println!("FLAGS: {}", cmd_flags);
        println!("ARGS: {:?}", args);

        // Handle unknown commands
        if let Err(e) = command.parse::<HshCmds>() {
            eprintln!("{}", e);
            continue;
        }
        let command = command.parse::<HshCmds>().unwrap();

        match HshCmds::from_input(command, cmd_flags) {
            Ok((HshCmds::Ls, cmd)) => {
                println!("{}", HshCmds::Ls);
                println!("Executing: {}", cmd.cmd)
            }
            Ok((HshCmds::Cd, cmd)) => {
                println!("Executing: {}", cmd.cmd)
            }
            Ok((HshCmds::Clear, cmd)) => {
                println!("Executing: {}", cmd.cmd)
            }
            Err(e) => eprintln!("{}", e)
        }

        // match command {
        //     "cd" => {
        //         let new_dir = args.first().map(|s| s.as_str()).unwrap_or("/");
        //         let root = Path::new(new_dir);
        //
        //         if args.len() != 1 {
        //             println!("Usage cd <directory>");
        //         } else if args.len() > 1 {
        //             println!("`cd` only accepts one argument")
        //         } else if let Err(e) = env::set_current_dir(root) {
        //             eprintln!("cd: {}", e);
        //         }
        //     }
        //
        //     "ls" => {
        //         let mut arg: Option<&str> = None;
        //         let mut dir: &str;
        //         if args.contains(&"-l".to_string()) {
        //             arg = Some("-l");
        //             println!("ls contains -l");
        //         }
        //         dir = ".";
        //         let _ = ls_dir(arg, dir);}
        //
        //     "set" => {
        //         if args.is_empty() {
        //             if vars.is_empty() {
        //                 println!("Not variables set.");
        //             } else {
        //                 for (key, value) in &vars {
        //                     println!("{}={}", key, value);
        //                 }
        //             }
        //         } else {
        //             for arg in args {
        //                 if let Some(eq_index) = arg.find('=') {
        //                     let key = &arg[..eq_index];
        //                     let value = &arg[eq_index + 1..];
        //                     vars.insert(key.to_string(), value.to_string());
        //                 } else {
        //                     println!("Invalid format: expected key=value")
        //                 }
        //             }
        //         }
        //     }
        //
        //     "pwd" => match env::current_dir() {
        //         Ok(path) => println!("{}", path.display()),
        //         Err(e) => println!("Error getting current directory: {}", e),
        //     },
        //
        //     "export" => {
        //         for arg in args {
        //             if let Some(eq_index) = arg.find('=') {
        //                 let key = &arg[..eq_index];
        //                 let value = &arg[eq_index + 1..];
        //                 vars.insert(key.to_string(), value.to_string());
        //                 unsafe { std::env::set_var(key, value) };
        //             } else {
        //                 println!("Invalid format in export: expected key=value got '{}'", arg);
        //             }
        //         }
        //     }
        //
        //     "unset" => {
        //         for key in &args {
        //             vars.remove(key);
        //             unsafe { std::env::remove_var(key) };
        //         }
        //     }
        //
        //     "env" => {
        //         for (key, value) in std::env::vars() {
        //             println!("{}={}", key, value);
        //         }
        //     }
        //
        //     "clear" => {
        //         let _ = clear_screen();
        //     }
        //
        //     "read" => {
        //         if args.len() != 1 {
        //             println!("Usage read <filename>");
        //         } else {
        //             let _ = read_command(args.first().unwrap());
        //         }
        //     }
        //
        //     "exit" => {
        //         break;
        //     }
        //
        //     "touch" => {
        //         let _ = touch_command(args.first().unwrap());
        //     }
        //
        //     "echo" => {
        //         println!("{}", args.join(" "));
        //     }
        //
        //     _ => {
        //         println!("Not Found!");
        //     }
        // }
    }
}

fn ls_dir(arg: Option<&str>, dir: &str) -> io::Result<()> {
    if let Some(arg) = arg {
        match arg {
        "-l" => println!("yaya contains '-l'"),
        _ => println!("invalid option")
    }}
    let mut list_dir = fs::read_dir(dir)?
        .map(|e| e.map(|res| res.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    list_dir.retain(|os_string| {
        if let Some(s) = os_string.to_str() {
            !s.starts_with(".")
        } else {
            false
        }
    });

    list_dir.sort();
    let mut output_string = String::new();
    for (index, item) in list_dir.iter().enumerate() {
        if let Some(s) = item.to_str() {
            output_string.push_str(s);
        }
        if index < list_dir.len() - 1 {
            output_string.push_str("    ");
        }
    }

    println!("{}", output_string);

    Ok(())
}

fn clear_screen() -> io::Result<()> {
    clearscreen::clear().unwrap();
    Ok(())
}

fn read_command(file_name: &str) -> io::Result<()> {
    let fopen = File::open(file_name);

    let mut f = match fopen {
        Ok(file) => file,

        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("oh dear file doesn't exist!");
                return Err(e);
            } else {
                // Handle other types of errors (e.g., permissions)
                println!("An error occurred while opening the file: {:?}", e);
                return Err(e); // Propagate other errors too
            }
        }
    };

    let mut buf = String::new();

    f.read_to_string(&mut buf)?;

    println!("{}", buf);
    Ok(())
}

fn touch_command(file_name: &str) -> io::Result<()> {
    let path = Path::new(file_name);

    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    Ok(())
}

fn resolve_variable(input: &str, vars: &HashMap<String, String>, flags: &mut Flags) -> Option<String> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            let mut var_name = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    var_name.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            if !var_name.is_empty() {
                if let Some(value) = vars.get(&var_name) {
                    result.push_str(value);
                }
            } else {
                result.push('$');
            }
        } else if c == '-' {
            while let Some(&next_c) = chars.peek() {
                flags.push(Flag::new(next_c));
                // eprintln!("c: {}, next_c: {}", c, next_c);
                chars.next();
            }
        } else {
            result.push(c);
        }
    }

    // Avoids building vec of empty strings
    if result.is_empty() {
        None 
    } else {
        Some(result)
    }
}

use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::Path,
};

fn main() {
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
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => {
                let new_dir = args.first().unwrap_or(&"/");
                let root = Path::new(new_dir);

                if args.len() != 1 {
                    println!("Usage cd <directory>");
                } else if args.len() > 1 {
                    println!("It does not take more than one argument.")
                } else {
                    if let Err(e) = env::set_current_dir(root) {
                        eprintln!("cd: {}", e);
                    }
                }
            }

            "ls" => {
                let _ = ls_dir();
            }

            "clear" => {
                let _ = clear_screen();
            }

            "read" => {
                if args.len() != 1 {
                    println!("Usage read <filename>");
                } else {
                    let _ = read_command(args.first().unwrap());
                }
            }

            "exit" => {
                break;
            }

            "touch" => {
                let _ = touch_command(args.first().unwrap());
            }

            _ => {
                println!("Not Found!")
            }
        }
    }
}

fn ls_dir() -> io::Result<()> {
    let mut list_dir = fs::read_dir(".")?
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
        },
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
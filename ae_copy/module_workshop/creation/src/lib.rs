pub mod start_script {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::path::PathBuf;

    pub fn find_root() -> Option<PathBuf> {
        let mut dir = std::env::current_dir().unwrap();

        loop {
            if dir.join("root.txt").exists() {
                return Some(dir);
            }

            if !dir.pop() {
                break;
            }
        }

        None
    }

    pub fn build() {
        let path = match find_root() {
            Some(path) => path,
            None => {
                let create_root =
                    match File::create((std::env::current_dir().unwrap()).join("root.txt")) {
                        Ok(_) => {
                            println!("Creating root id \"root.txt\"");
                            std::env::current_dir().unwrap()
                        }
                        Err(_) => panic!("Failed to create root id \"root.txt\""),
                    };

                create_root
            }
        };

        if !fs::exists(&path.join("src")).expect("Failed to check in build") {
            match fs::create_dir(&path.join("src")) {
                Ok(_) => println!("Created src dir"),
                Err(_) => panic!("Failed to create src dir"),
            }
        }
    }

    pub fn start() {
        let root = find_root().unwrap();
        let base_path = Path::new(&root).join("src/creations");

        if !fs::exists(&base_path).expect("failed to check") {
            match fs::create_dir(&base_path) {
                Ok(_) => println!("Created init dir"),
                Err(_) => panic!("Failed to create directory"),
            };

            let mut welcome_file = match File::create(&base_path.join("welcome.txt")) {
                Ok(file) => {
                    println!("Created welcome file");
                    file
                }
                Err(_) => panic!("Failed to create new project file"),
            };

            let welcome_text = format!("Hello this is the logs section of the app.\nHere you can log anything you want and come back to it later to edit as you go.\nTo start type the file name into the File Name text box then click New File.\nIt should now show the first log and a box you can type into to insert new text.\nLater on you can click the file name and press edit or read to edit or read the file.");
            welcome_file
                .write_all(welcome_text.as_bytes())
                .expect("Failed to write to init file");
            
        }
    }
}

pub mod file_handles {
    use super::start_script;
    use chrono::Local;
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::path::Path;

    pub fn find_creations() -> Vec<String> {
        let mut creations = vec![];

        let root = start_script::find_root().unwrap();
        let base_path = Path::new(&root).join("src/creations");

        let dir_walk = fs::read_dir(base_path).expect("Failed to walk dir");

        for entry in dir_walk {
            if let Ok(file) = entry {
                if let Some(name) = file.file_name().to_str() {
                    creations.push(name.to_string());
                }
            }
        }
        creations
    }

    pub fn new_creation(name: String) {
        let root = start_script::find_root().unwrap();
        let dir_path = Path::new(&root).join("src/creations/");
        let base_path = Path::new(&dir_path).join(&name);

        if fs::exists(base_path.clone()).expect("failed to check file") {
            println!("Creation already exists");
            return;
        }

        println!("Creating new project...");
        match File::create(&base_path) {
            Ok(_) => log_edit(name, true),
            Err(_) => panic!("Failed to create new project file"),
        }
    }

    pub fn log_handle(name: String, command: &str) {
        let root = start_script::find_root().unwrap();
        let dir_path = Path::new(&root).join("src/creations/");
        let base_path = Path::new(&dir_path).join(&name);

        match command {
            "r" => {
                let contents = fs::read_to_string(base_path).expect("Failed to read file");

                // clears the terminal
                print!("{esc}c", esc = 27 as char);

                println!("{}", contents);
            }
            "e" => {
                // clears the terminal
                print!("{esc}c", esc = 27 as char);

                log_handle(name.clone(), "r");
                log_edit(name, false);
            }
            _ => {
                panic!("Command not recognized")
            }
        }
    }

    pub fn log_edit(name: String, init: bool) {
        let root = start_script::find_root().unwrap();
        let dir_path = Path::new(&root).join("src/creations/");
        let base_path = Path::new(&dir_path).join(&name);

        // checking to see if it is first time creating this file
        if init {
            // write to file " Log 1 at {datetime}: "Created project {project_name}" "
            let now = Local::now();
            let date = now.date_naive();
            let time = now.time();

            let init_log = format!(
                "Log 1 on {} at {}: \"Created project {}\"",
                date, time, name
            );

            let mut edit_file = match File::create(base_path) {
                Ok(file) => file,
                Err(_) => panic!("Failed to open and init file"),
            };

            edit_file
                .write_all(init_log.as_bytes())
                .expect("Failed to write to init file");
            println!("Successfully created project file {}", name);

            return;
        }

        // normal editing of the file
        // this should continuously allow more logs to be added without exiting from editing the file until done

        let mut log_entry = String::new();

        loop {
            std::io::stdin()
                .read_line(&mut log_entry)
                .expect("Failed to read line");

            log_entry = log_entry.trim().to_string();

            if log_entry.eq("x") {
                break;
            }

            let file =
                BufReader::new(File::open(&base_path).expect("Failed to count project file"));
            let mut count = 1;
            let mut file_content = String::new();
            for line in file.lines() {
                file_content.push_str(line.unwrap().as_str());
                file_content.push_str("\n");
                count += 1;
            }

            let now = Local::now();
            let date = now.date_naive();
            let time = now.time();

            let new_log = format!("Log {} on {} at {}: \"{}\"", count, date, time, log_entry);
            file_content.push_str(new_log.as_str());

            let mut edit_file = match File::create(&base_path) {
                Ok(file) => file,
                Err(_) => panic!("Failed to open and edit file"),
            };

            edit_file
                .write_all(file_content.as_bytes())
                .expect("Failed to edit file");
            // log_handle(name.clone(), "r");

            log_entry.clear();
            file_content.clear();
        }
    }
}

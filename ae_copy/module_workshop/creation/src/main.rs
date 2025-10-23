use creation::{file_handles, start_script};

fn main() {
    start_script::build();
    start_script::start();

    let mut command = String::new();

    loop {
        let creations = file_handles::find_creations();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        command = command.trim().to_string();
        let new_creation = command.clone();
        let new_check = new_creation.split(" ").collect::<Vec<&str>>();
        if new_check[0] == "x" {
            break;
        } 
        else if new_check[1] == "n" {
            let mut new_project = new_check[0].to_string();
            new_project = new_project.trim().to_string();
            new_project.push_str(".txt");
            file_handles::new_creation(new_project.trim().to_string());
            
            file_handles::log_handle(new_project.trim().to_string(), "e");
            
            command.clear();

            continue;
            
        }

        let mut parts = command.split(" ").collect::<Vec<&str>>();

        let mut file = parts[0].trim().to_string();
        file.push_str(".txt");

        if parts.len() < 2 {
            parts.clear();
            file.clear();
            command.clear();
            continue;
        }

        let flag = parts[1].trim();


        if creations.contains(&file) {

            file_handles::log_handle(file.clone(), flag);

        }
        
        parts.clear();
        file.clear();
        command.clear();

    }
}

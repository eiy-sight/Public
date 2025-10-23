use chrono::Local;
use std::process::Command;
use std::{thread, time};

fn main() {
    let program_path = "~/AE/modules/learning_module/learning";

    let mut cur_hour: i8 = Local::now().format("%H").to_string().parse().unwrap();
    loop {
        let now = Local::now();

        thread::sleep(time::Duration::from_secs(1));

        if (now.format("%M").to_string() == "41")
            && cur_hour == now.format("%H").to_string().parse::<i8>().unwrap()
        {
            Command::new("gnome-terminal")
                .arg("--")
                .arg("bash")
                .arg("-c")
                .arg(format!("echo fetching question"))
                //.arg(format!("echo 'r' | {}; exec bash", program_path))
                .status()
                .expect("failed to open the terminal");

            cur_hour = Local::now().format("%H").to_string().parse().unwrap();
            cur_hour += 1;
        }
    }
}

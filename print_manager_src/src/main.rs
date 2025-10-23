use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::Deserialize;
use serde_json::json;
// use serde_json::Result;

#[derive(Default, Serialize, Deserialize, Clone)]
struct Json {
    name: String,
    time: String,
    layers: i32,
    length: f32,
    volume: f32,
    weight: f32,
    // cost: f32
}

fn project_init(project_path: &PathBuf) {
    let json_init = json!(
        [
            {
                "name": "INIT",
                "time": "0 minutes",
                "layers": 0,
                "length": 0.0,
                "volume": 0.0,
                "weight": 0.0
            }

        ]
        );
    match File::create(&project_path) {
        Ok(file) => {
            println!("creating new index file for project {:?}...", &project_path);
            serde_json::to_writer_pretty(file, &json_init).expect(""); 
        }
        Err(_) => {
            panic!("failed to create index file for {:?}", project_path);
        }
    }
}

fn find_old(old_values: &Vec<Json>) -> Vec<String> {
    let mut check_values: Vec<String> = Default::default();
        
    for value in old_values.iter() {
        println!("adding old {}", value.name);
        check_values.push(value.name.clone());
    }

    check_values

}


fn main() -> std::io::Result<()> {
    let mut init = 0;

    // in the final production search all of the print files and make an index for each different
    // make and version
    // /home/eiy/Projects/Prints
    
   

    // walk the projects dir and find any new projects and make a init index file for them

    let dir_read = fs::read_dir("/home/eiy/Projects/Prints").unwrap();
    let mut dir_paths = Vec::new();

    for value in dir_read {
        let formatted = value.unwrap();
        let current_index = Path::new(&formatted.path()).join("index.json");
        let path_value = formatted.path();
        match File::open(&current_index) {
            Ok(_) => {
                dir_paths.push(path_value);
                init = 1;
            },

            Err(_) => {
                project_init(&current_index);
                dir_paths.push(path_value);
            }
        }
    }


    // use the paths in dir_paths to find every file and put the info in the same index file
    let mut project_path = Vec::new();
    for path in dir_paths {
        let dir_read = fs::read_dir(&path).unwrap();
        for file_path in dir_read {
            if !file_path.as_ref().unwrap().path().ends_with("index.json") {
                let filtered_path = file_path.as_ref().unwrap().path();
                project_path.push(filtered_path);
            }
        }
    }


    // this code organizes the data and writes to the file

    for value in project_path.iter() {
        // this sets up the current index path for future reading
        //
        let mut index_path = PathBuf::new();
        if let Some(parent) = value.parent().clone() {
            index_path.push(parent);
            index_path.push("index.json");
        }

        let mut name: String = String::new();

        if let Some(file_name) = value.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                name = file_name_str.to_string();
            }
        }
        let f = File::open(value)?;

        let reader = BufReader::new(f);
        // gets the information I want
        //
        let mut counter = 0;
        let mut content = Vec::new();
        for line in reader.lines() {
            counter = counter + 1;
            match counter {
                3 => content.push(line.unwrap()),
                4 => content.push(line.unwrap()),
                5 => content.push(line.unwrap()),
                6 => content.push(line.unwrap()),
                7 => content.push(line.unwrap()),
                10 => break,
                _ => continue
            }
        }

        // this formats the information into a struct
        //
        let mut json_values: Json = Default::default(); // set the value that we will be writing
        json_values.name = String::from(&name);
        for (index, string) in content.iter().enumerate() {
            let pre_value = string.split(": ").collect::<Vec<&str>>();
            let value = pre_value[pre_value.len() - 1];
            match index {
                0 => json_values.time = value.to_string(),
                1 => json_values.layers = value.parse::<i32>().unwrap(),
                2 => json_values.length = value.parse::<f32>().unwrap(),
                3 => json_values.volume = value.parse::<f32>().unwrap(),
                4 => json_values.weight = value.parse::<f32>().unwrap(),
                _ => continue
            }
        }


        // get what is currently in the file
        let old_file = File::open(&index_path)?;
        let reader = BufReader::new(old_file);


        // get a vector JSON of old file
        let mut json_write: Vec<Json> = serde_json::from_reader(reader).expect("");
        let test_json = json_write.clone();


        // compare the current reading file to a list of file currently stored
        let check_values = find_old(&test_json);
        let current_file = &json_values.name;

        if !check_values.iter().any(|e| *e == *current_file) {
            json_write.push(json_values);
        }

//        println!("adding");
//        for value in json_write {
//            println!("{}", value.name);
//        }

        let new_write = File::create(&index_path)?;
        serde_json::to_writer_pretty(new_write, &json_write).expect("");

        
            //if !check_values.iter().any(|e| *e == value.name) {
            //    println!("Not yet");
            //    json_write.push(value);
            //}
        //}

        //for json in &converted {
        //    println!("{:?}", json.name);
        //    println!("{:?}", json.time);
        //    println!("{:?}", json.layers);
        //    println!("{:?}", json.length);
        //    println!("{:?}", json.volume);
        //    println!("{:?}", json.weight);
        //    println!("\n");
        //}
        //println!("\n");
        
        //let new_write = File::create(&index_path)?;
        //serde_json::to_writer_pretty(new_write, &json_write).expect("");
    }



    // next format the strings into something better
    // next add printing cost to the list 
    // finally append this information to the end of a file (type yet to be decided)
        

    Ok(())
}

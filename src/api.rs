use std::{env, fs};

pub fn list_db_files(path: &str) -> Vec<String> {
    let exe_path = env::current_exe().unwrap();
    let dir = exe_path.parent().unwrap();
    let path = dir.join(path);
    // println!("{} 66666666666666666666",&path.to_path_buf().as_os_str().to_string_lossy());
    fs::create_dir_all(&path).expect("Failed to create directory");
    let mut db_files = Vec::new();
    let files = fs::read_dir(path.to_path_buf().as_path()).unwrap();

    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.ends_with(".db") {
            db_files.push(file_name.to_string());
        }
    }

    db_files
}

pub fn formated_list_db_files(db_files:Vec<String>) -> Vec<String>{
    let formatted_dates: Vec<String> = db_files
    .iter()
    .map(|file| {
        let parts: Vec<&str> = file.split('-').collect();
        let month = parts[0];
        let year = parts[1].split('.').next().unwrap();
        let month_name = match month {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => unreachable!(),
        };
        format!("{} {}", month_name, year)
    })
    .collect();

    formatted_dates
}
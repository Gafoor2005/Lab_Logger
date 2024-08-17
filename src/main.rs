
use std::{collections::HashMap, fs::{self, File}, io::{self, BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, path::{self, Path}, process::Command, thread, time::Duration};

use api::{formated_list_db_files, list_db_files};
use chrono::{Datelike, Utc};
use config::get_config;
use crossterm::terminal::disable_raw_mode;
use dialoguer::Select;
use lab_logger::ThreadPool;
use regex::Regex;
use rpassword::read_password;
use rusqlite::Connection;
use database::Database;
mod config;

mod database;
mod api;



fn main() {
    display_branding();
    ctrlc::set_handler(move || {
        disable_raw_mode().unwrap();
        println!("\n\nExiting...");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
    get_config().unwrap();
    // println!("\x1b[32m✓ Green\x1b[0m");
    // println!("\x1b[34m✓ Blue\x1b[0m");
    // println!("\x1b[31m✓ Red\x1b[0m");
    // println!("\x1b[36m✓ cyan\x1b[0m");
    // println!("\x1b[33m✓ Yellow\x1b[0m");
    let items = vec![
        "new session", 
        "continue recent session", 
        "manage reports",
        "exit(^c)",
    ];
    let selection = Select::new()
        .with_prompt("\n\x1b[33mWhat do you choose? \x1b[34m(select using arrow keys ⇵)\x1b[0m")
        .items(&items)
        .interact()
        .unwrap();

    // println!("You chose: {}", items[selection]);
    if selection == 0{
        init_new_session();
    } else if selection == 1 {
        init_continue_session();
    } else if selection == 2 {
        open::that("http://localhost:5252").unwrap();
        start_server();
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn init_continue_session(){
    println!("\n------------- Select a recent session ---------------\n");

    let mut db = config_db();
    db.init().unwrap();

    let rows = db.choose_session();

    match rows {
        Ok(rows)=> {
            if rows.len()!=0{
                loop_input(db);
            }else {
                println!("\n\x1b[33m⚠  No Recent Sessions available ⚠\x1b[0m");
            }
        },
        Err(_) => return
    }


}

fn loop_input(db: Database) {
    loop {
        // println!("Enter roll number (or 'q' to quit):");
        std::io::stdout().flush().unwrap();
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        let input = read_password().unwrap();
        let input = input.trim();

        if input.to_lowercase() == "q" {
            break;
        }

        let pattern = Regex::new(r"^\d{5}[A-Z]{1}\d{2}[A-Z\d]\d$").unwrap();
        if pattern.is_match(input) {   
            let roll_number:&str  = input;
            let _ = db.add_student_record(roll_number);
        } else{
            print!("\x1b[31minvalid roll number!!\x1b[0m")
        }

    }
}

fn config_db() -> Database {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    let db_folder = "databases";

    let db_file_name = format!("{:02}-{}.db", month, year);
    let db_path = Path::new(db_folder).join(db_file_name);
    match fs::create_dir_all(db_folder) {
        Ok(_) => (),
        Err(err) => println!("\x1b[31mError creating folder: {}\x1b[0m", err),
    }
    
    let conn = Connection::open(db_path).unwrap();
    let db = Database { conn, table_name: "".to_string() };
    db
}

fn connect_database(db_file_name: &str) -> Result<Connection, rusqlite::Error> {
    let db_folder = "databases";
    let db_path = Path::new(db_folder).join(db_file_name);
    match fs::create_dir_all(db_folder) {
        Ok(_) => (),
        Err(err) => println!("\x1b[31mError creating folder: {}\x1b[0m", err),
    }
    let _ = File::open(&db_path).map_err(|_| rusqlite::Error::InvalidPath(db_path.clone()))?;
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

fn init_new_session() {
    println!("\n------------- Enter Session Details ---------------\n");

    let mut db = config_db();

    db.init().unwrap();

    // take input of session details
    print!("\x1b[33mEnter lab name \x1b[34m(course name eg:Java programming Lab)\x1b[0m: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let course = input.trim().to_string();
    
    print!("\x1b[33mEnter Branch name \x1b[34m(Dept name eg:CSE)\x1b[0m: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let branch = input.trim().to_string();

    let years = vec![
        "1st year",
        "2nd year",
        "3rd year",
        "4th year"
    ];
    let year = Select::new()
        .with_prompt("\x1b[33mSelect year \x1b[34m(select using arrow keys ⇵)\x1b[0m")
        .items(&years)
        .interact()
        .unwrap();
    let year = year+1;

    let semesters = vec![
        "1st semester",
        "2nd semester"
    ];
    let semester = Select::new()
        .with_prompt("\x1b[33mSelect semester \x1b[34m(select using arrow keys ⇵)\x1b[0m")
        .items(&semesters)
        .interact()
        .unwrap();
    let semester = semester+1;
    println!();

    let _ = db.new_session( course, branch.to_uppercase(), year.to_string(), semester.to_string());
    loop_input(db);
}

fn display_branding() {
    let text = "Wellcome to Lab Logger";
    let width = 40;
    
    // Print the top border
    for _ in 0..width {
        print!("\x1b[33m=");
    }
    println!();
    
    // Print the text with decorations
    println!("||{:^width$}||", text, width = width - 4);
    
    // Print the bottom border
    for _ in 0..width {
        print!("=");
    }
    println!("\n\n\x1b[36mDeveloped by Gafoor\x1b[0m\n\n");
}


fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:5252").unwrap();
    let pool = ThreadPool::new(4);
    

    for stream in listener.incoming() {
        let stream = stream.unwrap();
    
        pool.execute(|| {
            handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("handling.");
    let buf_reader = BufReader::new(&mut stream);
    let request_line_res = buf_reader.lines().next();
    if request_line_res.is_none() {return;}
    let request_line = request_line_res.unwrap().unwrap();
    println!("{}",request_line);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => 
            ("HTTP/1.1 200 OK", Path::new("web/index.html").to_str().unwrap().to_string()),
        s if s.starts_with("GET /?") => {
            ("HTTP/1.1 200 OK", Path::new("web/index.html").to_str().unwrap().to_string())
        }
        "GET /config HTTP/1.1" => 
            ("HTTP/1.1 200 OK", "config.json".to_string()),
        "GET /list_db HTTP/1.1" => ("HTTP/1.1 200 OK", "list_db".to_string()),
        s if s.starts_with("GET /db/") => {
            let path = request_line[5..].split_whitespace().nth(0).unwrap();
            println!("{}",path);
            ("HTTP/1.1 200 OK", path.to_string())
        }
        _ => {
            let req_path = request_line[5..].split_whitespace().nth(0).unwrap();
            let filename:Vec<&str> = req_path.split("?").collect();
            let req_path=filename[0];
            let path = Path::new("web/").join(req_path);
            println!("{} ",req_path);
            if path.exists() {
                ("HTTP/1.1 200 OK", path.to_str().unwrap().to_string())
            } else {
                send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                return;
            }
        }
    };

    // checking for db API
    if filename.starts_with("db") {
        let filename:Vec<&str> = filename.split("?").collect();
        let paths:Vec<&str> = filename[0].split("/").collect();

        // checking for db name in path
        if paths.len() < 2 {
            send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
            return;
        }

        // establishing conn to db
        let conn = match connect_database(&format!("{}.db",paths[1])) {
            Ok(conn) => conn,
            Err(err) => {
                println!("Error opening database: {}", err);
                // Return a default value or handle the error in some way
                // For example, you could return an empty connection or a error connection
                // Connection::default()
                send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());

                return;
            }
        };

        let mut table_name:String = "".to_string();
        // adding table name if exist
        if paths.len() > 2{
            table_name = paths[2].to_string()
        }
        // println!("{}",table_name.clone());

        // initializing database with connection and table name if exist
        let db = Database{conn,table_name};

        // if no table name then accessing master table
        if db.table_name.is_empty() {
            println!("accessing master only");
            let master_records = db.get_master_records();
            match master_records {
                Ok(master_records)=>{
                    let contents = match serde_json::to_string(&master_records) {
                        Ok(json) => json,
                        Err(err) => {
                            println!("Error converting to JSON: {}", err);
                            // Return a default value or handle the error in some way
                            send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                            return;
                        }
                    };
                    send_response(stream,  status_line, contents);
                }
                Err(err) => {
                    println!("Error converting to JSON: {}", err);
                    // Return a default value or handle the error in some way
                    send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                    return;
                }
            }
            

            return;
        }

        // else accessing session table with table name (table_name => which is already initialized to database)
        let student_records = db.get_student_records();
        let student_records = match student_records {
            Ok(rows) => rows,
            Err(err) => {
                println!("Error converting to JSON: {}", err);
                // Return a default value or handle the error in some way
                send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                return;
            }
        };
        let contents = match serde_json::to_string(&student_records) {
            Ok(json) => json,
            Err(err) => {
                println!("Error converting to JSON: {}", err);
                // Return a default value or handle the error in some way
                send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                return;
            }
        };
        send_response(stream,  status_line, contents);
        return;
    }

    if filename == "list_db" {
        let db_path = "databases";
        let db_list = list_db_files(&db_path);
        let formated_db_list = formated_list_db_files(db_list.clone());
        let mut map = HashMap::new();
        map.insert("files".to_string(), db_list);
        map.insert("formatted_files".to_string(), formated_db_list);
        let contents = match serde_json::to_string(&map) {
            Ok(json) => json,
            Err(err) => {
                println!("Error converting to JSON: {}", err);
                // Return a default value or handle the error in some way
                send_response(stream,  "HTTP/1.1 404 NOT FOUND", "haha not found".to_string());
                return;
            }
        };
        send_response(stream, status_line, contents);
        return;
    }
    // println!("---- this si {}",filename);
    let contents = fs::read_to_string(&filename);
    match contents {
        Ok(contents)=>send_response(stream, status_line, contents),
        Err(err) =>match err.kind() {
            std::io::ErrorKind::InvalidData => {
                // Handle InvalidData error
                let contents = fs::read(filename).unwrap();
                send_bit_response( stream, status_line, &contents);
            }
            _ => {
                // Handle other errors
                let error_message = "Internal Server Error";
                send_response(stream, "HTTP/1.1 500 Internal Server Error", error_message.to_string());
            }
        },
    }
    


}
    
fn send_response( mut stream: TcpStream,status_line:&str, contents: String) {
    let length = contents.len();
    
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
fn send_bit_response(mut stream: TcpStream, status_line: &str, contents: &[u8]) {
    let length = contents.len();

    let header = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    stream.write_all(header.as_bytes()).unwrap();
    stream.write_all(contents).unwrap();
}
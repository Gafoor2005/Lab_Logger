use dialoguer::Select;
use rusqlite::{Connection, Result};
use chrono::{NaiveDateTime, Timelike, Utc};
use chrono_tz::Asia::Kolkata;
use serde::Serialize;



// Define a struct to represent a student record
#[derive(Debug,Serialize)]
pub struct StudentRecord {
    roll_number: String,
    entry_timestamp: String,
    exit_timestamp: Option<String>,
}
#[derive(Serialize)]
pub struct MasterTableRecord {
    table_id: String,
    timestamp: String,
    course: String,
    branch: String,
    year: String,
    semester: String,
    session: String,
}

// Define a struct to represent the database
#[derive(Debug)]
pub struct Database {
    pub conn: Connection,
    pub table_name: String
}

impl Database {
    /// Method to initialize the database
    pub fn init(&mut self) -> Result<()> {

        // Create master_table if not available
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS master_table (
                table_id TEXT PRIMARY KEY,
                timestamp DATETIME,
                course TEXT,
                branch TEXT,
                year TEXT CHECK(year IN ('1', '2', '3','4')),
                semester TEXT CHECK(semester IN ('1', '2')),
                session TEXT CHECK(session IN ('forenoon', 'afternoon'))
            );
        ")?;


        Ok(())
    }

    /// creates new session
    pub fn new_session(&mut self,course:String,branch:String, year:String,semester:String) -> Result<()> {
        // Create a new session table with the current epoch millisecond timestamp as its name
        self.table_name = format!("s_{}", Utc::now().timestamp_millis());
        // self.table_name = "s_1723563266738".to_string();// for debugging

        // generate session an/fn
        let hour = Utc::now().with_timezone(&Kolkata).hour();
        let session = match hour < 12 {
            true => "forenoon".to_string(),
            false => "afternoon".to_string(),
        };

        // Create new session table and record this session in master
        self.conn.execute(&format!("CREATE TABLE {} (roll_number TEXT PRIMARY KEY, entry_timestamp DATETIME, exit_timestamp DATETIME)", &self.table_name), ())?;
        let master_table_name = "master_table";
        self.conn.execute(&format!("INSERT INTO {} (table_id, timestamp, course, branch, year, semester, session) VALUES (?, ?, ?, ?, ?, ?, ?)", master_table_name), 
            (&self.table_name, Utc::now().with_timezone(&Kolkata).format("%Y-%m-%d %H:%M:%S").to_string(), course, branch, year,semester, session))?;
        print!("\x1B[2J\x1B[1;1H");
        println!("\n\x1b[33mnew session started...\x1b[0m");
        Ok(())
    }


    /// Method to add a new student record
    pub fn add_student_record(&self, roll_number: &str) -> Result<()> {
        let entry_timestamp = Utc::now().with_timezone(&Kolkata);
        let result = self.conn.execute(&format!("INSERT OR IGNORE INTO {} (roll_number, entry_timestamp) VALUES (?, ?)", self.table_name), (roll_number, entry_timestamp.format("%Y-%m-%d %H:%M:%S").to_string()));

        match result {
            Ok(rows_affected) => {
                if rows_affected == 0 {
                    // println!("Row already exists.");
                    let _ = self.update_exit_timestamp(roll_number);
                } else {
                    // println!("Row inserted successfully.");
                    print!("\x1b[32m✓ {} entered the lab\x1b[0m",roll_number)
                }
            }
            Err(err) => {
                println!("\x1b[31mError: {}\x1b[0m", err);
            }
        }

        Ok(())
    }

    /// Method to update the exit timestamp for a given roll number
    fn update_exit_timestamp(&self, roll_number: &str) -> Result<()> {
        let exit_timestamp = Utc::now().with_timezone(&Kolkata);
        let entry_timestamp_str = self.conn.query_row(
            &format!("SELECT entry_timestamp FROM {} WHERE roll_number = ?", self.table_name),
            (&roll_number,),
            |row| row.get::<_,String>(0),
        )?;

        let entry_timestamp = NaiveDateTime::parse_from_str(&entry_timestamp_str, "%Y-%m-%d %H:%M:%S").unwrap();
        let duration = exit_timestamp.naive_local().signed_duration_since(entry_timestamp);
        // println!("{}", duration.num_minutes());
        if duration.num_minutes() >= 2 {
            self.conn.execute(
                &format!("UPDATE {} SET exit_timestamp = ? WHERE roll_number = ?", self.table_name),
                (exit_timestamp.format("%Y-%m-%d %H:%M:%S").to_string(), roll_number),
            )?;
            print!("\x1b[36m✓ {} exiting lab\x1b[0m",roll_number);
        }

        Ok(())
    }

    // select session from recent sessions
    pub fn choose_session(&mut self) -> Result<Vec<String>>{
        let today = Utc::now().with_timezone(&Kolkata).format("%Y-%m-%d").to_string();
        let mut stmt = self.conn.prepare("SELECT * FROM master_table WHERE DATE(timestamp) = ?")?;
        let rows = stmt.query_map([today], |row| {
            Ok(
                MasterTableRecord {
                    table_id: row.get::<_,String>(0)?,
                    timestamp: row.get::<_,String>(1)?,
                    course: row.get::<_,String>(2)?,
                    branch: row.get::<_,String>(3)?,
                    year: row.get::<_,String>(4)?,
                    semester: row.get::<_,String>(5)?,
                    session: row.get::<_,String>(6)?,
                }
            )
        })?;

        let records = rows.map(|row|->Result<MasterTableRecord,rusqlite::Error> {
            row
        }).collect::<Result<Vec<_>,_>>();
        let records = records.unwrap();
        //-----------------------display data in table-------------------

        // println!("{}{:20}{:20}{:30}{:5}{:10}{}{}\n",
        //  "\x1B[4m", "Table ID", "Timestamp", "Course", "Year", "Semester", "Session", "\x1B[0m");

        // for row in data {
        //     println!("{}{:20}{:20}{:30}{:5}{:10}{}\n",
        //      "\x1B[0m",row.table_id, row.timestamp, row.course, row.year, row.semester, row.session);
        // }

        let options:Vec<String> = (&records).into_iter().map(|row|{

            format!("{} | {:<9} | {}-{} | {:<10} | \x1b[33m{}\x1b[0m", row.timestamp,row.session,row.year,row.semester, row.branch,row.course)
        }).collect();

   
        
        // match data {
        //     Ok(data) => println!("{:?}", data),
        //     Err(err) => println!("Error: {}", err),
        // }
        if !options.is_empty(){

            let selection = Select::new()
                .with_prompt("\x1b[33mselect a session? \x1b[34m(select using arrow keys ⇵)\x1b[0m")
                .items(&options)
                .interact()
                .unwrap();

            // println!("Selected {}",records[selection].table_id);
            self.table_name = records[selection].table_id.to_string();
            print!("\x1B[2J\x1B[1;1H");
            println!("\n\x1b[33mThe session is active...\x1b[0m");
        }
        Ok(options)
        
    }

    pub fn get_master_records(&self) -> Result<Vec<MasterTableRecord>>{
        let mut stmt = self.conn.prepare("SELECT * FROM master_table ORDER BY timestamp DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(
                MasterTableRecord {
                    table_id: row.get::<_,String>(0)?,
                    timestamp: row.get::<_,String>(1)?,
                    course: row.get::<_,String>(2)?,
                    branch: row.get::<_,String>(3)?,
                    year: row.get::<_,String>(4)?,
                    semester: row.get::<_,String>(5)?,
                    session: row.get::<_,String>(6)?,
                }
            )
        })?;

        let records = rows.map(|row|->Result<MasterTableRecord,rusqlite::Error> {
            row
        }).collect::<Result<Vec<_>,_>>();
        
        records
    }
    pub fn get_student_records(&self) -> Result<Vec<StudentRecord>>{
        let mut stmt = self.conn.prepare(&format!("SELECT * FROM {}",&self.table_name))?;
        let rows = stmt.query_map([], |row| {
            Ok(
                StudentRecord {
                    roll_number: row.get::<_,String>(0)?,
                    entry_timestamp: row.get::<_,String>(1)?,
                    exit_timestamp: row.get::<_,Option<String>>(2)?,
                }
            )
        })?;
        let records = rows.map(|row|->Result<StudentRecord,rusqlite::Error> {
            row
        }).collect::<Result<Vec<_>,_>>();
        

        records
    }
}


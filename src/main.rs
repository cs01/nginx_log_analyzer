use chrono::NaiveDate;
use clap::{App, Arg};
use flate2::read::GzDecoder;
use glob::glob;
use regex::Regex;
use rusqlite::{params, Connection};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let matches = App::new("Nginx Log Analyzer")
        .version("1.0")
        .author("Chad Smith")
        .about("Analyzes Nginx logs to count unique visitors per day and provide a summary.")
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .value_name("PATTERN")
                .help("Sets the pattern to search for log files, including .gz files")
                .takes_value(true)
                .default_value("/var/log/nginx/access.log*"),
        )
        .arg(
            Arg::with_name("db")
                .short("d")
                .long("database")
                .value_name("DATABASE")
                .help("Sets the path to the SQLite database file")
                .takes_value(true)
                .default_value("nginxaccess.db"),
        )
        .arg(
            Arg::with_name("write-db")
                .long("write-db")
                .help("Enables writing the summary to the SQLite database")
                .takes_value(false),
        )
        .get_matches();

    let log_pattern = matches.value_of("pattern").unwrap();
    let log_regex = Regex::new(r"(\d{1,3}(?:\.\d{1,3}){3}) - - \[([^\]]+)\]").unwrap();
    let mut visitors: BTreeMap<NaiveDate, HashMap<String, bool>> = BTreeMap::new();

    for entry in glob(log_pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let reader: Box<dyn BufRead> = if path.extension().unwrap_or_default() == "gz" {
                    Box::new(BufReader::new(GzDecoder::new(File::open(path)?)))
                } else {
                    Box::new(BufReader::new(File::open(path)?))
                };

                for line in reader.lines() {
                    if let Ok(log) = line {
                        if let Some(caps) = log_regex.captures(&log) {
                            let ip = caps
                                .get(1)
                                .expect("Failed to match IP address in log entry")
                                .as_str()
                                .to_string();
                            let date_str = caps
                                .get(2)
                                .expect("Failed to match date string in log entry")
                                .as_str();
                            // Adjust date format as per your log
                            let date = NaiveDate::parse_from_str(date_str, "%d/%b/%Y:%H:%M:%S %z")
                                .unwrap_or_else(|_| panic!("Date format doesn't match: '{}' with format '%d/%b/%Y:%H:%M:%S %z'", date_str));

                            visitors
                                .entry(date)
                                .or_insert_with(HashMap::new)
                                .insert(ip, true);
                        }
                    }
                }
            }
            Err(e) => eprintln!("glob error: {:?}", e),
        }
    }

    // After processing the logs
    if matches.is_present("write-db") {
        let db_path = matches.value_of("db").unwrap();
        let db_exists = Path::new(db_path).exists();

        let conn = Connection::open(db_path).expect("Failed to open or create the database");

        if !db_exists {
            conn.execute(
                "CREATE TABLE visitors (
                        date TEXT PRIMARY KEY,
                        num_visitors INTEGER
                    )",
                [],
            )
            .expect("Failed to create table");
        }

        for (date, ips) in &visitors {
            let num_visitors = ips.len() as i32;
            conn.execute(
                "INSERT INTO visitors (date, num_visitors) VALUES (?1, ?2)
                    ON CONFLICT(date) DO UPDATE SET num_visitors=excluded.num_visitors",
                params![date.format("%Y-%m-%d").to_string(), num_visitors],
            )
            .expect("Failed to insert or update data in the database");
        }
    }

    // Print summary
    let mut total_unique_visitors = 0;
    for (date, ips) in &visitors {
        println!("{}: {} unique visitors", date, ips.len());
        total_unique_visitors += ips.len();
    }
    println!(
        "Total unique visitors across all days: {}, over {} days",
        total_unique_visitors,
        visitors.len()
    );

    Ok(())
}

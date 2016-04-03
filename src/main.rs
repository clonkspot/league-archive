extern crate league_archive;
extern crate clap;

use std::env;
use std::process::exit;
use std::path::Path;
use clap::App;
use league_archive::*;

fn get_env(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => {
            println!("{} not set", name);
            exit(1);
        }
    }
}

fn main() {
    let mysql_url = get_env("MYSQL_URL");

    let matches = App::new("League Archiver")
        .about("Creates a copy of the league MySQL tables as SQLite database")
        .args_from_usage(
            "<path> 'Output path of the SQLite database'
            -f      'Override the database if it exists'")
        .get_matches();

    let sqlite_path = Path::new(matches.value_of("path").unwrap());
    if sqlite_path.exists() {
        if matches.is_present("f") {
            std::fs::remove_file(sqlite_path).unwrap();
        } else {
            println!("File {} does already exist, use -f to overwrite.", sqlite_path.to_str().unwrap());
            std::process::exit(1);
        }
    }

    let mut archiver = Archiver::new(&mysql_url, &sqlite_path.to_str().unwrap()).unwrap();
    let result = archiver.copy_all().unwrap();
    println!("Done, copied {} rows.", result);
}

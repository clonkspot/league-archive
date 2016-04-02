extern crate league_archive;

use std::env;
use std::process::exit;
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
    let mut archiver = Archiver::new(&get_env("MYSQL_URL"), &get_env("SQLITE_DB")).unwrap();
    let result = archiver.copy_all().unwrap();
    println!("Done, copied {} rows.", result);
}

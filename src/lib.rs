extern crate mysql;
extern crate rusqlite;

mod tables;

use std::os::raw::c_int;

#[derive(Debug)]
pub enum Error {
    MySqlError(mysql::Error),
    SqliteError(rusqlite::Error),
}

impl From<mysql::Error> for Error {
    fn from(err: mysql::Error) -> Error {
        Error::MySqlError(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::SqliteError(err)
    }
}

pub struct Archiver {
    mysql: mysql::Conn,
    sqlite: rusqlite::Connection,
}

impl Archiver {
    // Constructs an archiver from the given database urls.
    pub fn new(mysql_url: &str, sqlite_url: &str) -> Result<Archiver, Error> {
        let mysql = try!(mysql::Conn::new(mysql_url));
        let sqlite = try!(rusqlite::Connection::open(sqlite_url));
        Ok(Archiver { mysql: mysql, sqlite: sqlite })
    }

    // Copies the given table from MySQL to sqlite, returning the number of rows copied.
    fn copy(&mut self, table: &tables::Table) -> Result<u64, Error> {
        let tx = try!(self.sqlite.transaction());
        try!(self.sqlite.execute(table.definition, &[]));
        let mut stmt = try!(self.sqlite.prepare(table.insert_sql));
        let mut n = 0;
        for row_result in try!(self.mysql.prep_exec(table.select_sql, ())) {
            let row = try!(row_result);
            // Rusqlite's ToSql trait manages the conversion of the different MySQL values.
            let params: Vec<MysqlValue> = row.unwrap().iter().map(|c| MysqlValue(c.clone())).collect();
            let params_slice: Vec<&ToSql> = params.iter().map(|v| v as &ToSql).collect();
            try!(stmt.execute(params_slice.as_slice()));
            n += 1;
        }
        try!(tx.commit());
        Ok(n)
    }

    // Copies all defined tables from MySQL to sqlite, returning the number of rows copied.
    pub fn copy_all(&mut self) -> Result<u64, Error> {
        let tables = vec![
            &tables::USERS,
            &tables::CLANS,
            &tables::CLAN_SCORES,
            &tables::GAMES,
            &tables::GAME_PLAYERS,
            &tables::GAME_SCORES,
            &tables::GAME_TEAMS,
            &tables::LEAGUES,
            &tables::SCORES,
        ];
        let mut n = 0;
        for table in tables {
            n += try!(self.copy(table));
        }
        Ok(n)
    }

}

use mysql::value::Value;
use rusqlite::types::ToSql;

struct MysqlValue(Value);

impl rusqlite::types::ToSql for MysqlValue {
    unsafe fn bind_parameter(&self, stmt: *mut rusqlite::types::sqlite3_stmt, col: c_int) -> c_int {
        let &MysqlValue(ref value) = self;
        match *value {
            Value::NULL => ToSql::bind_parameter(&rusqlite::types::Null, stmt, col),
            Value::Bytes(ref v) => ToSql::bind_parameter(v, stmt, col),
            Value::Int(ref i) => ToSql::bind_parameter(i, stmt, col),
            // Sqlite doesn't support u64
            Value::UInt(ref i) => ToSql::bind_parameter(&(*i as i64), stmt, col),
            Value::Float(ref i) => ToSql::bind_parameter(i, stmt, col),
            // Don't support date/time
            Value::Date(_, _, _, _, _, _, _) => panic!("mysql::value::Value::Date not supported"),
            Value::Time(_, _, _, _, _, _) => panic!("mysql::value::Value::Time not supported"),
        }
    }
}

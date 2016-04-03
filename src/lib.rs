extern crate encoding;
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
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1252;

struct MysqlValue(Value);

enum DecodeState {
    Char,
    Backslash,
    First(u8),
    Second(u8),
}

fn parse_octal(c: u8) -> Option<u8> {
    let ch = c as char;
    if ch < '0' || ch > '7' {
        None
    } else {
        Some(c - ('0' as u8))
    }
}

fn valid_octal_sequence(bytes: &Vec<u8>, pos: usize) -> bool {
    bytes.len() > pos + 3
        && bytes[pos] == ('\\' as u8)
        && parse_octal(bytes[pos + 1]).is_some()
        && parse_octal(bytes[pos + 2]).is_some()
        && parse_octal(bytes[pos + 3]).is_some()
}

// Decodes the league's octal Latin1 escapes, converting to UTF-8.
// Invalid escapes are left as-is.
fn decode_bytes(bytes: &Vec<u8>) -> String {
    // Resolve all octal escapes.
    let bytes: Vec<u8> = bytes.iter().enumerate().scan(DecodeState::Char, |state, (i, &c)| {
        let (new_state, out) = match *state {
            DecodeState::Char => {
                if valid_octal_sequence(bytes, i) {
                    (DecodeState::Backslash, None)
                } else {
                    (DecodeState::Char, Some(c))
                }
            },
            DecodeState::Backslash => (DecodeState::First(parse_octal(c).unwrap()), None),
            DecodeState::First(n)  => (DecodeState::Second((n << 3) + parse_octal(c).unwrap()), None),
            DecodeState::Second(n) => (DecodeState::Char, Some((n << 3) + parse_octal(c).unwrap()))
        };
        *state = new_state;
        Some(out)
    }).filter_map(|c| c ).collect();
    WINDOWS_1252.decode(bytes.as_slice(), DecoderTrap::Replace).unwrap()
}

#[test]
fn test_decode_bytes() {
    assert_eq!("Abwärts".to_string(), decode_bytes(&r"Abw\344rts".as_bytes().to_vec()));
    assert_eq!("Fußball".to_string(), decode_bytes(&r"Fu\337ball".as_bytes().to_vec()));
    assert_eq!("CrazyElevator 2¾".to_string(), decode_bytes(&r"CrazyElevator 2\276".as_bytes().to_vec()));
    assert_eq!("\\abc\"".to_string(), decode_bytes(&"\\abc\"".as_bytes().to_vec()));
    assert_eq!("äöüß".to_string(), decode_bytes(&r"\344\366\374\337".as_bytes().to_vec()));
}

impl rusqlite::types::ToSql for MysqlValue {
    unsafe fn bind_parameter(&self, stmt: *mut rusqlite::types::sqlite3_stmt, col: c_int) -> c_int {
        let &MysqlValue(ref value) = self;
        match *value {
            Value::NULL => ToSql::bind_parameter(&rusqlite::types::Null, stmt, col),
            Value::Bytes(ref v) => ToSql::bind_parameter(&decode_bytes(&v), stmt, col),
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

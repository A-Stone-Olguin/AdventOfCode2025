use rusqlite::{Connection, Result, Statement};
use rusqlite_migration::Migrations;
use include_dir::{include_dir, Dir};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub struct PreparedStatements<'conn> {
    insert_timings: Statement<'conn>,
}

pub fn init_db() -> Result<Connection, rusqlite::Error> {
    let database_file = format!("{}/db/results.db", env!("CARGO_MANIFEST_DIR"));
    let mut conn = Connection::open(database_file)?;
    let Ok(migrations) = Migrations::from_directory(&MIGRATIONS_DIR) else {
        panic!("Failed to get migrations from directory");
    };
    match migrations.to_latest(&mut conn) {
        Ok(_) => println!("Successfully applied migrations"),
        Err(e) => eprintln!("Failed to apply migrations: {}", e),
    }
    // TODO: FIX THIS
    // conn.execute("PRAGMA foreign_keys = ON;", [])?;
    // conn.execute("PRAGMA journal_mode='wal';", [])?;
    let _ = match prepare_stmts(&mut conn) {
        Ok(statements) => statements,
        Err(e) => return Err(e)
    };
    Ok(conn)
}

fn prepare_stmts<'conn>(conn: &'conn mut Connection) -> rusqlite::Result<PreparedStatements<'conn>, rusqlite::Error> {
    let insert_timings_stmt = match conn.prepare(
        "INSERT INTO timings (day_id, time_ms) VALUES (?1, ?2);"
    ) {
        Ok(statment) => statment,
        Err(e) => return Err(e),
    };
    Ok(PreparedStatements { insert_timings: insert_timings_stmt })
}


pub fn insert_timings<'conn>(day: u8, part: u8, timings_ms: Vec<f64>, prepared_statements: &mut PreparedStatements<'conn>) -> Result<(), rusqlite::Error> {
   let day_id: u32 = (2*(day-1) + (part-1)).into();
   for timing_ms in timings_ms {
        match prepared_statements.insert_timings.execute(rusqlite::params![day_id, timing_ms]) {
            Ok(_) => continue,
            Err(e) => return Err(e),
        }
   }
   Ok(())
}
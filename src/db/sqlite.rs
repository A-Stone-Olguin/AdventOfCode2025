use rusqlite::{Connection, Result, Rows, Statement};
use rusqlite_migration::Migrations;
use include_dir::{include_dir, Dir};

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub struct PreparedStatements<'conn> {
    insert_timings_stmt: Statement<'conn>,
    select_timings_stmt: Statement<'conn>,
    delete_timings_stmt: Statement<'conn>,
}

pub struct Sqlite {
    conn: Connection,
    // prepared_statements: PreparedStatements<'conn>
}

impl Sqlite {
    pub fn new() -> Result<Self> {
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
        Ok(Self { conn })
    }

    pub fn prepare_stmts(&mut self) -> Result<PreparedStatements<'_>> {
        let delete_timings_stmt = match self.conn.prepare("DELETE FROM timings WHERE day_id = ?;") {
            Ok(statement) => statement,
            Err(e) => return Err(e),
        };

        let insert_timings_stmt = match self.conn.prepare(
            "INSERT INTO timings (day_id, time_ms) VALUES (?1, ?2);"
        ) {
            Ok(statment) => statment,
            Err(e) => return Err(e),
        };

        let select_timings_stmt = match self.conn.prepare(
            r#"
            SELECT
                d.day,
                d.part,
                min(t.time_ms),
                avg(t.time_ms),
                max(t.time_ms),
                count(t.id)
            FROM days d
            LEFT JOIN timings t ON d.id = t.day_id
            GROUP BY t.day_id
            "#
        ) {
            Ok(statement) => statement,
            Err(e) => return Err(e),
        };

        Ok(PreparedStatements {
            insert_timings_stmt: insert_timings_stmt,
            select_timings_stmt: select_timings_stmt,
            delete_timings_stmt: delete_timings_stmt,
        })
    }
}

impl<'conn> PreparedStatements<'conn> {
    // TODO: Need to find out how to batch this
    pub fn insert_timings(&mut self, day: u8, part: u8, timings_ms: &Vec<i64>) -> Result<(), rusqlite::Error> {
        let day_id: u32 = (2*(day-1) + (part-1)).into();

        match self.delete_timings_stmt.execute(rusqlite::params![day_id]) {
            Ok(_) => println!("Deleted prior timing data for Day {} Part {}", day, part),
            Err(e) => return Err(e),
        }

        for timing_ms in timings_ms {
            match self.insert_timings_stmt.execute(rusqlite::params![day_id, timing_ms]) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn get_timings(&mut self) -> Result<Rows<'_>, rusqlite::Error> {
        self.select_timings_stmt.query([])
    }
}
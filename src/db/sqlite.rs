use rusqlite::{Connection, Result, Rows, Statement, functions::{Aggregate, FunctionFlags}};
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
        conn.create_aggregate_function("median", 1, FunctionFlags::SQLITE_DETERMINISTIC, MedianSqlite {})?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        Ok(Self { conn })
    }

    pub fn prepare_stmts(&mut self) -> Result<PreparedStatements<'_>> {
        let delete_timings_stmt = self.conn.prepare("DELETE FROM timings WHERE day_id = ?;")?;

        let insert_timings_stmt = self.conn.prepare(
            "INSERT INTO timings (day_id, time_ms) VALUES (?1, ?2);"
        )?;

        let select_timings_stmt = self.conn.prepare(
            r#"
            SELECT
                d.day,
                d.part,
                min(t.time_ms),
                median(t.time_ms),
                max(t.time_ms),
                count(t.id)
            FROM days d
            LEFT JOIN timings t ON d.id = t.day_id
            GROUP BY t.day_id
            "#
        )?;

        Ok(PreparedStatements {
            insert_timings_stmt,
            select_timings_stmt,
            delete_timings_stmt,
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

struct MedianSqlite {}

impl Aggregate<Vec<f64>, f64> for MedianSqlite {
    fn init(&self, _ctx: &mut rusqlite::functions::Context<'_>) -> Result<Vec<f64>> {
        Ok(Vec::new())
    }

    fn step(&self, ctx: &mut rusqlite::functions::Context<'_>, acc: &mut Vec<f64>) -> Result<()> {
        if let Ok(Some(value)) = ctx.get::<Option<f64>>(0) {
            acc.push(value);
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut rusqlite::functions::Context<'_>, acc: Option<Vec<f64>>) -> Result<f64> {
        let mut rows = match acc {
            Some(rows) => rows,
            None => return Ok(0.0),
        };

        if rows.is_empty() {
            return Ok(0.0)
        }
    
        rows.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = rows.len() / 2;
        if rows.len() % 2 == 0 {
            Ok((rows[mid - 1] + rows[mid]) / 2.0)
        } else {
            Ok(rows[mid])
        }
    }
}
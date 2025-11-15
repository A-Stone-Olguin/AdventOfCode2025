use rusqlite::{Connection, Result, functions::FunctionFlags};
use rusqlite_migration::Migrations;
use include_dir::{include_dir, Dir};
use crate::db::timing_result::TimingResult;
use crate::traits::timing_repository::TimingRepository;
use crate::db::sqlite::median::MedianSqlite;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

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
            Ok(_) => println!("Successfully applied migrations\n"),
            Err(e) => eprintln!("Failed to apply migrations: {e}"),
        }
        conn.create_aggregate_function("median", 1, FunctionFlags::SQLITE_DETERMINISTIC, MedianSqlite {})?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;

        Ok(Self { conn })
    }
}

impl TimingRepository<rusqlite::Error> for Sqlite {
    // TODO: Batch insert
    fn insert_timings(&mut self, day_id: u8, timings_ms: &[i64]) -> rusqlite::Result<()> {
        let mut insert_timings_stmt = self.conn.prepare_cached(
            "INSERT INTO timings (day_id, time_ms) VALUES (?1, ?2);"
        )?;

        for timing_ms in timings_ms {
            match insert_timings_stmt.execute(rusqlite::params![day_id, timing_ms]) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn delete_day_timings(&mut self, day_id: u8) -> rusqlite::Result<usize> {
        let mut delete_timings_stmt = self.conn.prepare_cached("DELETE FROM timings WHERE day_id = ?;")?;
        delete_timings_stmt.execute(rusqlite::params![day_id])
    }

    fn get_timings(&mut self) -> rusqlite::Result<Vec<TimingResult>> {
        let mut select_timings_stmt = self.conn.prepare_cached(
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
            ORDER BY d.day;
            "#
        )?;

        let mut rows = select_timings_stmt.query([])?;

        let mut timing_results: Vec<TimingResult> = vec![];
        while let Some(row) = rows.next()? {
            let result = TimingResult {
                day: row.get(0)?,
                part: row.get(1)?,
                min_time_ms: row.get(2).unwrap_or_default(),
                median_time_ms: row.get(3).unwrap_or_default(),
                max_time_ms: row.get(2).unwrap_or_default(),
                number_iterations: row.get(5).unwrap_or_default(),
            };
            timing_results.push(result);
        }
        Ok(timing_results)
    }
}
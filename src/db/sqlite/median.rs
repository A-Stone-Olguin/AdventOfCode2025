

use rusqlite::{Result, functions::Aggregate};

pub struct MedianSqlite {}

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
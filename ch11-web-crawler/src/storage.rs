use crate::types::CrawlResult;
use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::thread::JoinHandle;
use tokio::sync::mpsc;
use tracing::info;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS pages (
    url        TEXT PRIMARY KEY,
    status     INTEGER NOT NULL,
    html       TEXT    NOT NULL,
    fetched_at INTEGER NOT NULL
);";

pub fn spawn_writer(
    db_path: PathBuf,
    mut rx: mpsc::Receiver<CrawlResult>,
) -> JoinHandle<Result<usize>> {
    std::thread::spawn(move || -> Result<usize> {
        let mut conn = Connection::open(&db_path)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.execute_batch(SCHEMA)?;
        let mut written = 0usize;
        while let Some(first) = rx.blocking_recv() {
            let tx = conn.transaction()?;
            insert_one(&tx, &first)?;
            written += 1;
            for _ in 0..99 {
                match rx.try_recv() {
                    Ok(r) => {
                        insert_one(&tx, &r)?;
                        written += 1;
                    }
                    Err(_) => break,
                }
            }
            tx.commit()?;
        }
        info!(written, "db writer finished");
        Ok(written)
    })
}

fn insert_one(tx: &rusqlite::Transaction, r: &CrawlResult) -> rusqlite::Result<()> {
    tx.execute(
        "INSERT OR IGNORE INTO pages (url, status, html, fetched_at) \
         VALUES (?1, ?2, ?3, ?4)",
        params![
            r.page.url.as_str(),
            r.page.status,
            r.page.html,
            r.page.fetched_at
        ],
    )?;
    Ok(())
}

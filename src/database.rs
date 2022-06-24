use anyhow::Result;
use rusqlite::{params, Connection};

use super::Event;

pub fn add_event(conn: &Connection, event: &Event) -> Result<()> {
    conn.execute(
        "INSERT INTO events ( date, hash, project_id ) VALUES ( ?, ?, ? )",
        params![event.date, event.hash, event.project_id],
    )?;

    Ok(())
}

pub fn add_project(conn: &Connection, project_name: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO projects ( name ) VALUES ( ? )",
        params![project_name],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn cleanup_database(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id FROM projects")?;
    let project_list: Vec<i64> = stmt
        .query_map([], |row| row.get::<usize, i64>(0))?
        .filter_map(|x| x.ok())
        .collect();

    for id in project_list {
        let c: i32 = conn.query_row(
            "SELECT COUNT ( * ) FROM events WHERE project_id = ?",
            params![id],
            |row| row.get(0),
        )?;

        if c == 0 {
            conn.execute("DELETE FROM projects WHERE id = ?", params![id])?;
        }
    }

    Ok(())
}

pub fn get_event_count(conn: &Connection, project_id: i64) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT COUNT ( * ) FROM events WHERE project_id = ?",
        params![project_id],
        |row| row.get(0),
    )?)
}

pub fn get_events(conn: &Connection, project_id: i64) -> Result<Vec<Event>> {
    let mut stmt = conn.prepare("SELECT date, hash FROM events WHERE project_id = ?")?;
    let event_list = stmt
        .query_map(params![project_id], |row| {
            Ok(Event {
                date: row.get(0)?,
                hash: row.get(1)?,
                project_id,
            })
        })?
        .filter_map(|row| row.ok())
        .collect();

    Ok(event_list)
}

pub fn get_projects(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM projects")?;
    let project_list: Vec<String> = stmt
        .query_map([], |row| Ok(row.get(0)?))?
        .filter_map(|row| row.ok())
        .collect();

    Ok(project_list)
}

pub fn get_project_id(conn: &Connection, project_name: &str) -> Result<i64> {
    Ok(conn.query_row(
        "SELECT id FROM projects WHERE name = ?",
        params![project_name],
        |row| row.get::<usize, i64>(0),
    )?)
}

pub fn remove_event(conn: &Connection, hash: &str) -> Result<()> {
    conn.execute("DELETE FROM events WHERE hash = ?", params![hash])?;

    Ok(())
}

pub fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY, 
            project_id INTEGER NOT NULL, 
            date INTEGER NOT NULL,
            hash TEXT NOT NULL,
            FOREIGN KEY ( 
                project_id 
            ) REFERENCES projects ( 
                id 
            )
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects ( 
            id INTEGER PRIMARY KEY, 
            name TEXT NOT NULL 
        )",
        [],
    )?;

    Ok(())
}

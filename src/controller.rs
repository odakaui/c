use super::database;
use super::Event;
use anyhow::Result;
use chrono::TimeZone;
use chrono::Local;
use rusqlite::Connection;

pub fn add(conn: &Connection, project_name: &str) -> Result<()> {
    let date = Local::now().timestamp();
    let event = create_event(conn, date, project_name)?;

    database::add_event(conn, &event)?;

    print_count(conn, project_name)?;

    Ok(())
}

pub fn count(conn: &Connection, project_name: &str) -> Result<()> {
    print_count(conn, project_name)?;

    Ok(())
}

pub fn list(conn: &Connection, project_name: &str) -> Result<()> {
    let project_id = database::get_project_id(conn, project_name)?;
    let event_list = database::get_events(conn, project_id)?;

    for e in event_list {
        let date = Local.timestamp(e.date, 0).format("%Y-%m-%d %H:%M:%S");
        println!("{} {}", e.hash, date);
    }

    Ok(())
}

pub fn list_projects(conn: &Connection) -> Result<()> {
    let project_list = database::get_projects(conn)?;

    for p in project_list {
        println!("{}", p);
    }

    Ok(())
}

pub fn remove(conn: &Connection, hash: &str) -> Result<()> {
    database::remove_event(conn, hash)?;
    database::cleanup_database(conn)?;

    Ok(())
}

fn create_event(conn: &Connection, date: i64, project_name: &str) -> Result<Event> {
    let project_id = match database::get_project_id(conn, project_name) {
        Ok(id) => id,
        Err(_) => database::add_project(conn, project_name)?,
    };

    Event::new(project_id, date)
}

fn print_count(conn: &Connection, project_name: &str) -> Result<()> {
    let project_id = database::get_project_id(conn, project_name)?;
    let count = database::get_event_count(conn, project_id)?;

    println!("total: {}", count);

    Ok(())
}

use crate::controller::{add, count, list, list_projects, remove};
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use event::Event;
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::{path::PathBuf, fs::create_dir_all};

mod cli;
mod controller;
mod database;
mod event;

lazy_static! {
    static ref DATABASE_PATH: PathBuf = dirs::home_dir().unwrap().join(".local/share/c/c.db");
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !DATABASE_PATH.exists() {
        create_dir_all(DATABASE_PATH.parent().unwrap())?;
    }

    let conn = Connection::open(&*DATABASE_PATH)?;
    database::setup_database(&conn)?;

    match args.command {
        Command::Add { project_name } => add(&conn, &project_name)?,
        Command::Count { project_name } => count(&conn, &project_name)?,
        Command::List { project_name } => list(&conn, &project_name)?,
        Command::ListProjects => list_projects(&conn)?,
        Command::Remove { hash } => remove(&conn, &hash)?,
    }

    Ok(())
}

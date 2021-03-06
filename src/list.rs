use std::io;
use std::fs::{self, DirEntry};
use std::os::unix::fs::PermissionsExt;
use chrono::prelude::*;
use chrono::TimeZone;
use colored::*;
use std::time::{UNIX_EPOCH};
use std::path::PathBuf;

pub fn list(path: PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap();
        let meta = entry.metadata()?;
        
        if meta.is_dir() {
            println!("{}", name.bold());
        } else {
            println!("{}", name);
        }
    }

    Ok(())
}

pub fn list_long(path: PathBuf) -> io::Result<()> {
    let count = fs::read_dir(".")?.count();
    println!("total {:?}", count);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let mut filename = entry.file_name().into_string().unwrap();
        let meta = entry.metadata()?;
        let modified = meta.modified()?;
        let secs = modified.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let dt = Utc.timestamp(secs as i64, 0);
        let formatted = dt.format("%b %d %H:%M").to_string();
        let len = meta.len();
        let perm = meta.permissions();
        let mode = crate::mode::format_mode(perm.mode());

        if meta.is_dir() {
            filename = filename.bold().to_string()
        }

        println!("{} {:5} {} {}", mode, len, formatted, filename);
    }

    Ok(())
}

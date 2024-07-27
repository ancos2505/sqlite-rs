use sqlite_rs::{SqliteConnection, IN_MEMORY_URI};

use crate::sqlite_cli::result::SqliteCliResult;

use super::traits::PrintHelp;

pub(super) struct ReplOpen;
impl ReplOpen {
  pub(super) fn run(maybe_arg1: Option<String>) -> SqliteCliResult<SqliteConnection> {
    let uri_str: String;

    let conn = match maybe_arg1 {
      Some(conn_str) => {
        uri_str = if conn_str.contains("://") {
          match &*conn_str {
            "" | IN_MEMORY_URI => IN_MEMORY_URI.into(),
            s => {
              if s.contains(":") {
                IN_MEMORY_URI.into()
              } else {
                conn_str
              }
            }
          }
        } else {
          if conn_str.contains(":") {
            IN_MEMORY_URI.into()
          } else {
            format!("sqlite://{conn_str}")
          }
        };

        SqliteConnection::open(&uri_str)?
      }
      None => {
        uri_str = IN_MEMORY_URI.into();

        SqliteConnection::open(&uri_str)?
      }
    };
    println!("Connected: [{uri_str}]");
    Ok(conn)
  }
}
impl PrintHelp for ReplOpen {
  fn help() -> SqliteCliResult<()> {
    let help = [
      ".open ?OPTIONS? ?FILE?   Close existing database and reopen FILE",
      // TODO
      // "Options:",
      // "--new           Initialize FILE to an empty database",
      // "--nofollow      Do not follow symbolic links",
      // "--readonly      Open FILE readonly",
    ];
    help.iter().for_each(|line| println!("{line}"));
    Ok(())
  }
}

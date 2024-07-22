use super::traits::PrintHelp;
use crate::sqlite_cli::result::SqliteCliResult;
use sqlite_rs::{io::SqliteIo, pager::SqlitePager, runtime::SqliteRuntime};

pub(super) struct ReplOpen;
impl ReplOpen {
  pub(super) fn run(maybe_arg1: Option<String>) -> SqliteCliResult<SqliteRuntime> {
    let conn = match maybe_arg1 {
      Some(file_path) => {
        let io = SqliteIo::open(file_path)?;
        let pager = SqlitePager::connect(io)?;

        SqliteRuntime::start(pager)?
      }
      None => {
        let io = SqliteIo::open(":memory:")?;
        let pager = SqlitePager::connect(io)?;
        SqliteRuntime::start(pager)?
      }
    };
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

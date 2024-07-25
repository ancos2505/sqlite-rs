use crate::sqlite_cli::result::SqliteCliResult;

pub(super) fn run(normalized_input: impl AsRef<str>) -> SqliteCliResult<()> {
  if normalized_input.as_ref().len() > 0 {
    println!(r#"SQL queries is not implemented. Enter ".help" for help"#);
  }
  Ok(())
}

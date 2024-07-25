//! Tests for Pager
//!
//! To run: `cargo test pager`

#[test]

fn ok_on_check_page_size() {
  use crate::runtime::SqliteRuntime;
  use crate::SqliteConnection;
  let res = SqliteConnection::open("sqlite://./data/flights-initial.db");

  debug!("{res:?}");
  assert!(res.is_ok());

  let conn = res.unwrap();

  debug!("{conn:?}");

  assert_eq!(*conn.file_header().page_size(), 4096);
}

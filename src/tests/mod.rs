use crate::{
  debug,
  file_header::{self, MagicHeaderString},
  io::SqliteIoMode,
};

#[test]
fn ok_on_get_conn_methods() {
  use crate::SqliteConnection;
  let res = SqliteConnection::open("sqlite://./data/small.sqlite3");

  debug!("{res:?}");
  assert!(res.is_ok());

  let conn = res.unwrap();

  debug!("{conn:?}");

  let file_header = conn.file_header();
  assert_eq!(*file_header.page_size(), 512);

  let file_metadata = conn.file_metadata();
  assert!(file_metadata.is_some());

  let io_mode = conn.io_mode();
  assert_eq!(*io_mode, SqliteIoMode::File);
}

#[test]
fn ok_check_database_consistence_size() {
  // TODO: Need to consider freelists, etc.
  use crate::SqliteConnection;
  let sample_databases = [
    "./data/flights-initial.db",
    "./data/flights-populated.db",
    "./data/flights-deleted.db",
    "./data/mydatabase.db",
    "./data/small.sqlite3",
  ];
  sample_databases.iter().for_each(|file_path| {
    debug!("Testing database: {file_path}");
    dbg!(file_path);
    let uri = format!("sqlite://{file_path}");

    let res = SqliteConnection::open(uri);

    trace!("{res:?}");
    assert!(res.is_ok());

    let conn = res.unwrap();

    debug!("{conn:?}");

    let file_header = conn.file_header();

    let page_size = file_header.page_size();

    let pages = file_header.db_filesize_in_pages();

    let file_metadata = conn.file_metadata();

    assert_eq!(*conn.io_mode(), SqliteIoMode::File);
    file_metadata.map(|metadata| {
      let expected_size = u32::from(page_size) * **pages;
      assert_eq!(metadata.len(), expected_size.into());
    });

    assert_eq!(*conn.io_mode(), SqliteIoMode::File);
  });
}

// #[test]
// #[ignore = "Todo"]
// fn ok_on_new_inmemory_database() {
//   let res = SqliteConnection::open(IN_MEMORY_URI);
//   debug!("{res:?}");
//   assert!(res.is_ok());
//   let mut conn = res.unwrap();
//   debug!("{conn:?}");
//   let page = conn.runtime_mut().pager_mut().first().unwrap();
//   trace!("{page:?}");
//   let header = conn.runtime().header();
//   assert_eq!(header.page_size(), conn.runtime().pager().page_size());
//   assert_eq!(
//     header.reserved_bytes_per_page(),
//     conn.runtime().pager().reserved_bytes_per_page()
//   );
//   debug!("{header:?}");
// }

// #[test]
// fn ok_on_read_first_page() {
//   let res = SqliteConnection::open("sqlite://./data/small.sqlite3");
//   debug!("{res:?}");
//   assert!(res.is_ok());
//   let mut conn = res.unwrap();
//   debug!("{conn:?}");
//   let header = conn.runtime().file_header();
//   assert_eq!(header.page_size(), conn.runtime().pager().page_size());
//   assert_eq!(
//     header.reserved_bytes_per_page(),
//     conn.runtime().file_header().reserved_bytes_per_page()
//   );
//   debug!("{header:?}");

//   // TODO
//   let first_page = conn.runtime().pager().get_first_page().unwrap();
//   // let size = first_page.size();
//   // let data = first_page.data();
//   // let page_kind = first_page.kind();

//   // trace!("{size:?}");
//   // trace!("{data:?}");
//   // trace!("{page_kind:?}");
// }

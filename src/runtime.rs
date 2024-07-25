mod internal_tables;
mod schema;

use std::{fmt::Debug, fs::Metadata};

use crate::{
  file_header::SqliteHeader,
  io::{SqliteIo, SqliteIoMode},
  pager::SqlitePager,
  result::SqliteResult,
  traits::ParseBytes,
};

pub use self::schema::SqliteSchema;

pub struct SqliteRuntime {
  pager: SqlitePager,
  file_header: SqliteHeader,
}

impl Debug for SqliteRuntime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SqliteRuntime")
      .field("pager", &"SqlitePager")
      .field("header", &self.file_header)
      .finish()
  }
}

impl SqliteRuntime {
  pub fn start(conn_str: impl AsRef<str>) -> SqliteResult<Self> {
    trace!("Openning SQliteIo [{}]...", conn_str.as_ref());
    let io = SqliteIo::open(conn_str)?;
    trace!("SQliteIo started: [{io:?}].");
    trace!("Connecting SqlitePager...");

    let mut pager = SqlitePager::connect(io)?;
    trace!("SQliteIo started: [{pager:?}].");

    match pager.io().mode() {
      &SqliteIoMode::InMemory => Ok(Self {
        pager,
        file_header: Default::default(),
      }),
      _ => {
        let page = pager.get_first_page()?;
        trace!("RetrievedPage: [{page:?}].");
        let bytes = page.data();

        let file_header = SqliteHeader::parse_bytes(bytes).map_err(|err| {
          error!("{err}");
          err
        })?;
        Ok(Self { pager, file_header })
      }
    }
  }

  pub fn file_header(&self) -> &SqliteHeader {
    &self.file_header
  }

  pub fn pager(&self) -> &SqlitePager {
    &self.pager
  }

  pub fn file_metadata(&self) -> Option<&Metadata> {
    self.pager.io().file_metadata()
  }
}

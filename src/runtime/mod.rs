mod internal_tables;
mod schema;

use std::fmt::Debug;

use crate::{header::SqliteHeader, pager::SqlitePager, result::SqliteResult};

pub use self::schema::SqliteSchema;

pub struct SqliteRuntime {
  pager: SqlitePager,
  header: SqliteHeader,
}

impl Debug for SqliteRuntime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SqliteRuntime")
      .field("pager", &"SqlitePager")
      .field("header", &self.header)
      .finish()
  }
}

impl SqliteRuntime {
  pub fn start(pager: SqlitePager) -> SqliteResult<Self> {
    // let header = if pager.io_mut().is_empty()? {
    //   SqliteHeader::default()
    // } else {
    //   SqliteHeader::parse_bytes(pager.first()?.data().raw_data())?
    // };
    // TODO
    Ok(Self {
      pager,
      header: Default::default(),
    })
  }

  pub fn pager(&self) -> &SqlitePager {
    &self.pager
  }

  pub fn header(&self) -> &SqliteHeader {
    &self.header
  }
}

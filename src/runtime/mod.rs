mod btree;
mod internal_tables;
mod schema;

use self::btree::SqliteBtree;
use crate::{
  header::SqliteHeader, pager::SqlitePager, result::SqliteResult,
  traits::ParseBytes,
};

pub use self::schema::SqliteSchema;

#[derive(Debug)]
pub struct SqliteRuntime {
  pager: SqlitePager,
  header: SqliteHeader,
  btree: SqliteBtree,
}

impl SqliteRuntime {
  pub fn start(mut pager: SqlitePager) -> SqliteResult<Self> {
    let header = if pager.io_mut().is_empty()? {
      SqliteHeader::default()
    } else {
      SqliteHeader::parse_bytes(pager.first()?.raw_data())?
    };

    let btree = Default::default();
    Ok(Self {
      pager,
      header,
      btree,
    })
  }

  pub fn header(&self) -> &SqliteHeader {
    &self.header
  }

  pub fn tables(&self) -> SqliteResult<Vec<SqliteSchema>> {
    todo!("Show tables not implemented")
  }

  pub fn pager(&self) -> &SqlitePager {
    &self.pager
  }

  pub fn pager_mut(&mut self) -> &mut SqlitePager {
    &mut self.pager
  }
}

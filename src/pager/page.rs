//! # Pages
//!
//!  The main database file consists of one or more pages. The size of a page is
//! a power of two between 512 and 65536 inclusive. All pages within the same
//! database are the same size. The page size for a database file is determined
//! by the 2-byte integer located at an offset of 16 bytes from the beginning of
//! the database file.
//!
//!  Pages are numbered beginning with 1. The maximum page number is 4294967294
//! (232 - 2). The minimum size SQLite database is a single 512-byte page. The
//! maximum size database would be 4294967294 pages at 65536 bytes per page or
//! 281,474,976,579,584 bytes (about 281 terabytes). Usually SQLite will hit the
//! maximum file size limit of the underlying filesystem or disk hardware long
//! before it hits its own internal size limit.
//!
//!  In common use, SQLite databases tend to range in size from a few kilobytes
//! to a few gigabytes, though terabyte-size SQLite databases are known to exist
//! in production.
//!
//!  At any point in time, every page in the main database has a single use
//! which is one of the following:
//!
//! - The lock-byte page
//! - A freelist page
//!     - A freelist trunk page
//!     - A freelist leaf page
//! - A b-tree page
//!     - A table b-tree interior page
//!     - A table b-tree leaf page
//!     - An index b-tree interior page
//!     - An index b-tree leaf page
//! - A payload overflow page
//! - A pointer map page
//! - The lock-byte page
//!
//!  All reads from and writes to the main database file begin at a page
//! boundary and all writes are an integer number of pages in size. Reads are
//! also usually an integer number of pages in size, with the one exception that
//! when the database is first opened, the first 100 bytes of the database file
//! (the database file header) are read as a sub-page size unit.
//!
//!  Before any information-bearing page of the database is modified, the
//! original unmodified content of that page is written into the rollback
//! journal. If a transaction is interrupted and needs to be rolled back, the
//! rollback journal can then be used to restore the database to its original
//! state. Freelist leaf pages bear no information that would need to be
//! restored on a rollback and so they are not written to the journal prior to
//! modification, in order to reduce disk I/O.
//!
//!  All reads from and writes to the main database file begin at a page
//! boundary and all writes are an integer number of pages in size. Reads are
//! also usually an integer number of pages in size, with the one exception that
//! when the database is first opened, the first 100 bytes of the database file
//! (the database file header) are read as a sub-page size unit.
//!
//! *Reference:* https://www.sqlite.org/fileformat2.html#pages

mod btree;
mod freelist;
mod lock_byte;
mod payload_overflow;
mod pointer_map;

#[cfg(test)]
mod tests;

use std::{any::Any, fmt::Debug};

use btree::BtreePage;

use crate::result::SqliteResult;

pub struct Page<const N: usize> {
  pub(super) size: usize,
  pub(super) data: Box<[u8; N]>,
  pub(super) kind: PageKind,
}

impl<const N: usize> Page<N> {
  pub fn parse(input: &[u8]) -> SqliteResult<Self> {
    let page = input.try_into().map(|data: [u8; N]| Page {
      size: data.len(),
      data: Box::new(data),
      kind: PageKind::_Todo,
    })?;
    dbg!(&page);
    let btree_page = BtreePage::parse(*page.data)?;
    dbg!(btree_page);
    Ok(page)
  }

  pub fn size(&self) -> usize {
    self.size
  }

  pub fn data(&self) -> &[u8; N] {
    &self.data
  }
  pub fn kind(&self) -> &PageKind {
    &self.kind
  }
  pub fn from_dyn(page_any: Box<dyn Any>) -> Option<Self> {
    page_any.downcast::<Self>().ok().map(|p| *p)
  }
}

impl<const N: usize> Debug for Page<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Page")
      .field("size", &self.size)
      .field("data", &"[..]")
      .field("kind", &self.kind)
      .finish()
  }
}

/// # PageKind
///  At any point in time, every page in the main database has a single use
/// which is one of the following:
/// - A b-tree page
///     - A table b-tree interior page
///     - A table b-tree leaf page
///     - An index b-tree interior page
///     - An index b-tree leaf page
/// - A payload overflow page
/// - A pointer map page
/// - The lock-byte page
#[derive(Debug)]
pub enum PageKind {
  // TODO
  _Todo,
  TableBtreeInterior,
  TableBtreeLeaf,
  IndexBtreeInterior,
  IndexBtreeLeaf,
  PayloadOverflow,
  PointerMap,
  LockByte,
}

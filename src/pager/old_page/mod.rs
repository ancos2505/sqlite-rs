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
//! *Reference:* https://www.sqlite.org/fileformat2.html#pages

mod btree;
mod freelist;
mod lock_byte;
mod payload_overflow;
mod pointer_map;

#[cfg(test)]
mod tests;

use crate::{header::PageSize, result::SqliteResult};

#[derive(Debug)]
pub struct Page {
  size: PageSize,
  data: PageData,
  kind: PageKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PageData(Vec<u8>);
impl PageData {
  pub fn raw_data(&self) -> &Vec<u8> {
    &self.0
  }
}
impl Page {
  pub const MAX_LENGTH: usize = PageSize::MAX.as_usize();

  pub fn parse<const N: usize>(size: PageSize, raw_data: [u8; N]) -> SqliteResult<Self> {
    let data: PageData = {
      match (&size, raw_data.len()) {
        (&PageSize::L512, 512)
        | (&PageSize::L1024, 1024)
        | (&PageSize::L2048, 2048)
        | (&PageSize::L4096, 4096)
        | (&PageSize::L8192, 8192)
        | (&PageSize::L16384, 16384)
        | (&PageSize::L32768, 32768)
        | (&PageSize::L65536, 65536) => PageData(raw_data.to_vec()),
        _ => {
          return Err(crate::result::SqliteError::Custom(
            "Error on parsing Page. Bytes length does no match with PageSize".into(),
          ))
        }
      }
    };
    // TODO
    Ok(Self {
      size,
      data,
      kind: PageKind::_Todo,
    })
  }

  pub fn data(&self) -> &PageData {
    &self.data
  }
}

/// # PageKind
///  At any point in time, every page in the main database has a single use
/// which is one of the following:
/// - The lock-byte page
/// - A freelist page
///     - A freelist trunk page
///     - A freelist leaf page
/// - A b-tree page
///     - A table b-tree interior page
///     - A table b-tree leaf page
///     - An index b-tree interior page
///     - An index b-tree leaf page
/// - A payload overflow page
/// - A pointer map page
#[derive(Debug)]
enum PageKind {
  // TODO
  _Todo,
  LockByte,
  FreelistTrunk,
  FreelistLeaf,
  TableBtreeInterior,
  TableBtreeLeaf,
  IndexBtreeInterior,
  IndexBtreeLeaf,
  PayloadOverflow,
  PointerMap,
}

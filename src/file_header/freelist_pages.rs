//! # The Freelist
//!  A database file might contain one or more pages that are not in active use.
//! Unused pages can come about, for example, when information is deleted from
//! the database. Unused pages are stored on the freelist and are reused when
//! additional pages are required.
//!
//!  The freelist is organized as a linked list of freelist trunk pages with
//! each trunk page containing page numbers for zero or more freelist leaf
//! pages.
//!
//!  A freelist trunk page consists of an array of 4-byte big-endian integers.
//! The size of the array is as many integers as will fit in the usable space of
//! a page. The minimum usable space is 480 bytes so the array will always be at
//! least 120 entries in length. The first integer on a freelist trunk page is
//! the page number of the next freelist trunk page in the list or zero if this
//! is the last freelist trunk page. The second integer on a freelist trunk page
//! is the number of leaf page pointers to follow. Call the second integer on a
//! freelist trunk page L. If L is greater than zero then integers with array
//! indexes between 2 and L+1 inclusive contain page numbers for freelist leaf
//! pages.
//!
//!  Freelist leaf pages contain no information. SQLite avoids reading or
//! writing freelist leaf pages in order to reduce disk I/O.
//!
//!  A bug in SQLite versions prior to 3.6.0 (2008-07-16) caused the database to
//! be reported as corrupt if any of the last 6 entries in the freelist trunk
//! page array contained non-zero values. Newer versions of SQLite do not have
//! this problem. However, newer versions of SQLite still avoid using the last
//! six entries in the freelist trunk page array in order that database files
//! created by newer versions of SQLite can be read by older versions of SQLite.
//!
//!  The number of freelist pages is stored as a 4-byte big-endian integer in
//! the database header at an offset of 36 from the beginning of the file. The
//! database header also stores the page number of the first freelist trunk page
//! as a 4-byte big-endian integer at an offset of 32 from the beginning of the
//! file.

use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # Free page list (8 Bytes) => First(4 Bytes) + TotalPages (4 Bytes)
///  Unused pages in the database file are stored on a freelist.
#[derive(Debug, Default)]
pub struct FreeListPages {
  /// Page number of the first freelist trunk page. (4 Bytes)
  first: FreeListPagesFirstTrunkPage,
  /// Total number of freelist pages. (4 Bytes)
  total: FreeListPagesTotalPages,
}

impl FreeListPages {
  pub fn first(&self) -> &FreeListPagesFirstTrunkPage {
    &self.first
  }

  pub fn total(&self) -> &FreeListPagesTotalPages {
    &self.total
  }
}
impl_name! {FreeListPages}

impl ParseBytes for FreeListPages {
  const LENGTH_BYTES: usize = 8;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let first = FreeListPagesFirstTrunkPage::parse_bytes(&bytes[0..=3])?;
    let total = FreeListPagesTotalPages::parse_bytes(&bytes[4..=7])?;

    Ok(Self { first, total })
  }
}

///  FreeListPagesFirstTrunkPage: The 4-byte big-endian integer at offset 32
/// stores the page number of the first page of the freelist, or zero if the
/// freelist is empty.
#[derive(Debug, Default)]
pub struct FreeListPagesFirstTrunkPage(u32);
impl Deref for FreeListPagesFirstTrunkPage {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {FreeListPagesFirstTrunkPage}

impl ParseBytes for FreeListPagesFirstTrunkPage {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;
    let first_page_trunk = u32::from_be_bytes(buf);
    Ok(Self(first_page_trunk))
  }
}

///  FreeListPagesTotalPages: The 4-byte big-endian integer at offset 36
/// stores the total number of pages on the freelist.
#[derive(Debug, Default)]
pub struct FreeListPagesTotalPages(u32);
impl Deref for FreeListPagesTotalPages {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {FreeListPagesTotalPages}

impl ParseBytes for FreeListPagesTotalPages {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;
    let total_pages = u32::from_be_bytes(buf);
    Ok(Self(total_pages))
  }
}

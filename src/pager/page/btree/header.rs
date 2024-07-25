//! # B-tree page header
//!
//!  The b-tree page header is 8 bytes in size for leaf pages and 12 bytes for
//! interior pages. All multibyte values in the page header are big-endian. The
//! b-tree page header is composed of the following fields:
//!
//! |Offset | Size  | Description |
//! |-------|-------|-------------|
//! |  0    | 1     | The one-byte flag at offset 0 indicating the b-tree page type. |
//! |       |       |    - A value of 2 (0x02) means the page is an interior index b-tree page. |
//! |       |       |    - A value of 5 (0x05) means the page is an interior table b-tree page. |
//! |       |       |    - A value of 10 (0x0a) means the page is a leaf index b-tree page. |
//! |       |       |    - A value of 13 (0x0d) means the page is a leaf table b-tree page. |
//! |       |       | Any other value for the b-tree page type is an error.
//! | 1     | 2     | The two-byte integer at offset 1 gives the start of the first freeblock on the page, or is zero if there are no freeblocks. |
//! | 3     | 2     | The two-byte integer at offset 3 gives the number of cells on the page. |
//! | 5     | 2     | The two-byte integer at offset 5 designates the start of the cell content area. A zero value for this integer is interpreted as 65536. |
//! | 7     | 1     | The one-byte integer at offset 7 gives the number of fragmented free bytes within the cell content area. |
//! | 8     | 4     | The four-byte page number at offset 8 is the right-most pointer. This value appears in the header of interior b-tree pages only and is omitted from all other pages. |

pub(super) mod first_freeblock;
pub(super) mod number_of_cells;
pub(super) mod page_type;
pub(super) mod start_of_cell_content_area;

use std::num::NonZeroU16;

use crate::{
  field_parsing_error, impl_name,
  result::SqliteResult,
  traits::{Name, ParseBytes},
};

#[derive(Debug)]
pub struct BtreePageHeader {
  page_type: BtreePageType,
  // first_freeblock: Option<FirstFreeBlock>,
  // number_of_cells: NumberOfCells,
  // start_of_cell_content_area: StartOfContentArea,
  // number_of_fragmented: NumberOfFragmented,
  // TODO: ParseBytes was implemented for handling fixed length data structures  // right_most_pointer: Option<RightMostPointer>,
}

impl BtreePageHeader {
  pub fn parse(bytes: &[u8]) -> SqliteResult<Self> {
    dbg!(bytes);
    let page_type = BtreePageType::parse_bytes(&bytes[0..=0])?;
    dbg!(&page_type);
    Ok(Self { page_type })
  }
}

/// ### BtreePageType (1 Byte)
///
/// The one-byte flag at offset 0 indicating the b-tree page type.
///
/// - A value of 2 (0x02) means the page is an **interior index** b-tree page.
/// - A value of 5 (0x05) means the page is an **interior table** b-tree page.
/// - A value of 10 (0x0a) means the page is a **leaf index** b-tree page.
/// - A value of 13 (0x0d) means the page is a **leaf table** b-tree page.
///
/// Any other value for the b-tree page type is an error.
#[derive(Debug)]
pub enum BtreePageType {
  InteriorIndex,
  InteriorTable,
  LeafIndex,
  LeafTable,
}

impl_name!(BtreePageType);

impl ParseBytes for BtreePageType {
  const LENGTH_BYTES: usize = 1;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let maybe_byte = bytes.get(0);
    let outcome = maybe_byte
      .and_then(|byte| match *byte {
        2 => Some(Self::InteriorIndex),
        5 => Some(Self::InteriorTable),
        10 => Some(Self::LeafIndex),
        13 => Some(Self::LeafTable),
        _ => None,
      })
      .ok_or(field_parsing_error!(Self::NAME.into()))?;

    Ok(outcome)
  }
}

/// #### FirstFreeBlock (2 Bytes)
///  The two-byte integer at offset 1 gives the start of the first freeblock on
/// the page, or is zero if there are no freeblocks.
#[derive(Debug)]
pub struct FirstFreeBlock(NonZeroU16);

/// #### NumberOfCells (2 Bytes)
/// The two-byte integer at offset 3 gives the number of cells on the page.
#[derive(Debug)]
pub struct NumberOfCells(u16);

/// #### StartOfContentArea (2 Bytes)
///  The two-byte integer at offset 5 designates the start of the cell content
/// area. A zero value for this integer is interpreted as 65536.
#[derive(Debug)]
pub struct StartOfContentArea(u32);

/// #### NumberOfFragmented (1 Bytes)
///  The one-byte integer at offset 7 gives the number of fragmented free bytes
/// within the cell content area.
#[derive(Debug)]
pub struct NumberOfFragmented(u8);

/// #### RightMostPointer (4 Bytes)
///  The four-byte page number at offset 8 is the right-most pointer. This value
/// appears in the header of interior b-tree pages only and is omitted from all
/// other pages.
#[derive(Debug)]
pub struct RightMostPointer(u32);

use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// Incremental vacuum settings (8 Bytes)
///
///  The two 4-byte big-endian integers at offsets 52 and 64 are used to manage
/// the auto_vacuum and incremental_vacuum modes. If the integer at offset 52
/// is zero then pointer-map (ptrmap) pages are omitted from the database file
/// and neither auto_vacuum nor incremental_vacuum are supported. If the integer
/// at offset 52 is non-zero then it is the page number of the largest root page
/// in the database file, the database file will contain ptrmap pages, and the
/// mode must be either auto_vacuum or incremental_vacuum. In this latter case,
/// the integer at offset 64 is true for incremental_vacuum and false for
/// auto_vacuum. If the integer at offset 52 is zero then the integer at
/// offset 64 must also be zero.
#[derive(Debug, Default)]
pub struct IncrementalVacuumSettings {
  pub largest_root_btree_page: LargestRootBtreePage,
  pub incremental_vacuum_mode: IncrementalVacuumMode,
}

// TODO:  If the integer at offset 52 is non-zero then it is the page number of
// TODO: the largest root page in the database file, the database file will
// TODO: contain ptrmap pages, and the mode must be either auto_vacuum or
// TODO: incremental_vacuum.

impl IncrementalVacuumSettings {
  pub fn largest_root_btree_page(&self) -> &LargestRootBtreePage {
    &self.largest_root_btree_page
  }

  pub fn incremental_vacuum_mode(&self) -> &IncrementalVacuumMode {
    &self.incremental_vacuum_mode
  }
}

///  #  Largest root b-tree page (4 Bytes)
/// The page number of the largest root b-tree page when in auto-vacuum
/// or incremental-vacuum modes, or zero otherwise.
#[derive(Debug, Default)]
pub struct LargestRootBtreePage(u32);

impl Deref for LargestRootBtreePage {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {LargestRootBtreePage}

impl ParseBytes for LargestRootBtreePage {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let value = u32::from_be_bytes(buf);

    Ok(Self(value))
  }
}

/// # Incremental-vacuum mode (4 Bytes)
/// True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
#[derive(Debug, Default)]
pub enum IncrementalVacuumMode {
  #[default]
  False,
  True,
}
impl From<&IncrementalVacuumMode> for bool {
  fn from(value: &IncrementalVacuumMode) -> Self {
    match value {
      IncrementalVacuumMode::True => true,
      IncrementalVacuumMode::False => false,
    }
  }
}
impl From<&IncrementalVacuumMode> for u32 {
  fn from(value: &IncrementalVacuumMode) -> Self {
    match value {
      IncrementalVacuumMode::True => 1,
      IncrementalVacuumMode::False => 0,
    }
  }
}

impl_name! {IncrementalVacuumMode}

impl ParseBytes for IncrementalVacuumMode {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let number = u32::from_be_bytes(buf);
    let value = if number > 0 { Self::True } else { Self::False };

    Ok(value)
  }
}

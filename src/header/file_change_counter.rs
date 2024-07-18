use crate::traits::ParseBytes;
use crate::{impl_name, result::SqliteResult};
use core::ops::Deref;

/// # File change counter (4 Bytes)
///
///  The file change counter is a 4-byte big-endian integer at offset 24 that is
/// incremented whenever the database file is unlocked after having been
/// modified. When two or more processes are reading the same database file,
/// each process can detect database changes from other processes by monitoring
/// the change counter. A process will normally want to flush its database page
/// cache when another process modified the database, since the cache has become
/// stale. The file change counter facilitates this.
///
/// In WAL mode, changes to the database are detected using the wal-index and so
/// the change counter is not needed. Hence, the change counter might not be
/// incremented on each transaction in WAL mode.
#[derive(Debug, Default)]
pub struct FileChangeCounter(u32);
impl Deref for FileChangeCounter {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl_name! {FileChangeCounter}

impl ParseBytes for FileChangeCounter {
  const LENGTH_BYTES: usize = 4;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    Ok(Self(u32::from_be_bytes(buf)))
  }
}

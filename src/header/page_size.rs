use crate::traits::ParseBytes;
use crate::{
  impl_name,
  result::{SqliteError, SqliteResult},
};

/// # Page Size (2 Bytes)
///
///  The two-byte value beginning at offset 16 determines the page size of the
/// database. For Sqlite versions 3.7.0.1 (2010-08-04) and earlier, this value
/// is interpreted as a big-endian integer and must be a power of two between
/// 512 and 32768, inclusive. Beginning with Sqlite version 3.7.1 (2010-08-23),
/// a page size of 65536 bytes is supported. The value 65536 will not fit in a
/// two-byte integer, so to specify a 65536-byte page size, the value at offset
/// 16 is 0x00 0x01. This value can be interpreted as a big-endian 1 and
/// thought of as a magic number to represent the 65536 page size. Or one can
/// view the two-byte field as a little endian number and say that it
/// represents the page size divided by 256. These two interpretations of the
/// page-size field are equivalent.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum PageSize {
  L512,
  L1024,
  L2048,
  /// Reference: https://www.sqlite.org/pragma.html#pragma_page_size
  #[default]
  L4096,
  L8192,
  L16384,
  L32768,
  L65536,
}

impl PageSize {
  pub const MAX: Self = Self::L65536;

  pub fn iter() -> PageSizeIterator {
    PageSizeIterator {
      current: Some(PageSize::L512),
    }
  }
}

impl From<&PageSize> for u32 {
  fn from(value: &PageSize) -> Self {
    match *value {
      PageSize::L512 => 512,
      PageSize::L1024 => 1024,
      PageSize::L2048 => 2048,
      PageSize::L4096 => 4096,
      PageSize::L8192 => 8192,
      PageSize::L16384 => 16384,
      PageSize::L32768 => 32768,
      PageSize::L65536 => 65536,
    }
  }
}
impl PartialEq<usize> for PageSize {
  fn eq(&self, other: &usize) -> bool {
    match self {
      PageSize::L512 => *other == 512,
      PageSize::L1024 => *other == 1024,
      PageSize::L2048 => *other == 2048,
      PageSize::L4096 => *other == 4096,
      PageSize::L8192 => *other == 8192,
      PageSize::L16384 => *other == 16384,
      PageSize::L32768 => *other == 32768,
      PageSize::L65536 => *other == 655536,
    }
  }
}

impl PartialEq<PageSize> for usize {
  fn eq(&self, other: &PageSize) -> bool {
    match other {
      PageSize::L512 => *self == 512,
      PageSize::L1024 => *self == 1024,
      PageSize::L2048 => *self == 2048,
      PageSize::L4096 => *self == 4096,
      PageSize::L8192 => *self == 8192,
      PageSize::L16384 => *self == 16384,
      PageSize::L32768 => *self == 32768,
      PageSize::L65536 => *self == 655536,
    }
  }
}

impl_name! {PageSize}

impl ParseBytes for PageSize {
  const LENGTH_BYTES: usize = 2;

  fn parsing_handler(bytes: &[u8]) -> SqliteResult<Self> {
    let buf: [u8; Self::LENGTH_BYTES] = bytes.try_into()?;

    let page_size = u16::from_be_bytes(buf);

    match page_size {
      0 | 2..=511 => Err(SqliteError::Custom(
        "PageSize can't be less than 512".into(),
      )),
      512 => Ok(Self::L512),
      1024 => Ok(Self::L1024),
      2048 => Ok(Self::L2048),
      4096 => Ok(Self::L4096),
      8192 => Ok(Self::L8192),
      16384 => Ok(Self::L16384),
      32768 => Ok(Self::L32768),
      1 => Ok(Self::L65536),
      _ => Err(SqliteError::Custom("PageSize must be power of two".into())),
    }
  }
}

pub struct PageSizeIterator {
  current: Option<PageSize>,
}

impl Iterator for PageSizeIterator {
  type Item = PageSize;

  fn next(&mut self) -> Option<Self::Item> {
    let current = self.current.take()?;

    let new_current = match current {
      PageSize::L512 => Some(PageSize::L1024),
      PageSize::L1024 => Some(PageSize::L2048),
      PageSize::L2048 => Some(PageSize::L4096),
      PageSize::L4096 => Some(PageSize::L8192),
      PageSize::L8192 => Some(PageSize::L16384),
      PageSize::L16384 => Some(PageSize::L32768),
      PageSize::L32768 => Some(PageSize::L65536),
      PageSize::L65536 => None,
    };

    let value = current;

    self.current = new_current;

    Some(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ok_on_convert_pagesize_into_iterator() {
    let vec = PageSize::iter().map(|i| (&i).into()).collect::<Vec<u32>>();

    let expected = vec![512, 1024, 2048, 4096, 8192, 16384, 32768, 65536];

    assert_eq!(vec, expected);
  }
}
